use std::collections::HashMap;
use std::sync::Arc;

use wayland_client::backend::ObjectData;
use wayland_client::protocol::wl_registry;
use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    Connection, Dispatch, Proxy, QueueHandle,
};

// Protocol imports
use wayland_protocols::ext::foreign_toplevel_list::v1::client::{
    ext_foreign_toplevel_handle_v1, ext_foreign_toplevel_list_v1,
};
use cosmic_protocols::toplevel_info::v1::client::{
    zcosmic_toplevel_handle_v1, zcosmic_toplevel_info_v1,
};

struct DiscoveryState {
    // Keep handles alive
    cosmic_handles: HashMap<u32, zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1>,
    ext_handles: HashMap<u32, ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1>,

    // ext handle id -> app_id
    ext_app_ids: HashMap<u32, String>,

    // cosmic handle id -> ext handle id
    cosmic_to_ext: HashMap<u32, u32>,

    cosmic_info: Option<zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1>,
}

// --- DISPATCH LOGIC ---

// 1) ext-foreign-toplevel-list: new window discovered
impl Dispatch<ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1, ()> for DiscoveryState {
    fn event(
        state: &mut Self,
        _: &ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1,
        event: ext_foreign_toplevel_list_v1::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let ext_foreign_toplevel_list_v1::Event::Toplevel { toplevel } = event {
            // Keep ext handle alive so we keep receiving its events (AppId, Title, etc.)
            let ext_id = toplevel.id().protocol_id();
            state.ext_handles.insert(ext_id, toplevel.clone());

            // Create + keep cosmic handle alive for focus/activated state
            if let Some(mgr) = &state.cosmic_info {
                let cosmic_handle = mgr.get_cosmic_toplevel(&toplevel, qh, ());
                let cosmic_id = cosmic_handle.id().protocol_id();

                state.cosmic_to_ext.insert(cosmic_id, ext_id);
                state.cosmic_handles.insert(cosmic_id, cosmic_handle);
            }
        }
    }

    // REQUIRED: child proxy creation for ext handles
    fn event_created_child(_opcode: u16, qh: &QueueHandle<Self>) -> Arc<dyn ObjectData> {
        qh.make_data::<ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1, ()>(())
    }
}

// 2) ext handle events: capture AppId here (this is the key fix)
impl Dispatch<ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1, ()> for DiscoveryState {
    fn event(
        state: &mut Self,
        proxy: &ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        event: ext_foreign_toplevel_handle_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let ext_id = proxy.id().protocol_id();

        match event {
            // This is what you actually want
            ext_foreign_toplevel_handle_v1::Event::AppId { app_id } => {
                state.ext_app_ids.insert(ext_id, app_id);
            }

            // Cleanup
            ext_foreign_toplevel_handle_v1::Event::Closed => {
                state.ext_app_ids.remove(&ext_id);
                state.ext_handles.remove(&ext_id);

                // Also remove any cosmic handle that mapped to this ext handle
                if let Some((cosmic_id, _)) = state
                    .cosmic_to_ext
                    .iter()
                    .find(|(_, v)| **v == ext_id)
                    .map(|(k, v)| (*k, *v))
                {
                    state.cosmic_to_ext.remove(&cosmic_id);
                    state.cosmic_handles.remove(&cosmic_id);
                }
            }

            _ => {}
        }
    }
}

// 3) COSMIC handle events: use Activated bit for focus, but print ext AppId
impl Dispatch<zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1, ()> for DiscoveryState {
    fn event(
        state: &mut Self,
        proxy: &zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1,
        event: zcosmic_toplevel_handle_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let cosmic_id = proxy.id().protocol_id();

        match event {
            zcosmic_toplevel_handle_v1::Event::State { state: raw_bytes } => {
                let is_activated = raw_bytes
                    .chunks_exact(4)
                    .any(|c| u32::from_ne_bytes([c[0], c[1], c[2], c[3]]) == 2);

                if is_activated {
                    let ext_id = state.cosmic_to_ext.get(&cosmic_id).copied();

                    let app = ext_id
                        .and_then(|eid| state.ext_app_ids.get(&eid))
                        .cloned()
                        .unwrap_or_else(|| "<app_id not yet received>".to_string());

                    println!("WINDOW FOCUS CHANGED: {} (cosmic_id={}, ext_id={:?})", app, cosmic_id, ext_id);
                }
            }

            zcosmic_toplevel_handle_v1::Event::Closed => {
                state.cosmic_handles.remove(&cosmic_id);
                state.cosmic_to_ext.remove(&cosmic_id);
            }

            _ => {}
        }
    }
}

// 4) Boilerplate
impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for DiscoveryState {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, ()> for DiscoveryState {
    fn event(
        _: &mut Self,
        _: &zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1,
        _: zcosmic_toplevel_info_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }

    // REQUIRED: get_cosmic_toplevel() creates zcosmic_toplevel_handle objects
    fn event_created_child(_opcode: u16, qh: &QueueHandle<Self>) -> Arc<dyn ObjectData> {
        qh.make_data::<zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1, ()>(())
    }
}

// --- MAIN LOOP ---

fn main() -> anyhow::Result<()> {
    let conn = Connection::connect_to_env()?;
    let (globals, mut queue) = registry_queue_init::<DiscoveryState>(&conn)?;
    let qh = queue.handle();

    let cosmic_info = globals.bind::<zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, _, _>(
        &qh,
        1..=3,
        (),
    )?;
    let _list = globals.bind::<ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1, _, _>(
        &qh,
        1..=1,
        (),
    )?;

    let mut state = DiscoveryState {
        cosmic_handles: HashMap::new(),
        ext_handles: HashMap::new(),
        ext_app_ids: HashMap::new(),
        cosmic_to_ext: HashMap::new(),
        cosmic_info: Some(cosmic_info),
    };

    println!("Listening for COSMIC window changes...");

    loop {
        queue.blocking_dispatch(&mut state)?;
    }
}

use std::collections::BTreeMap;
use std::fs;
use std::time::Duration;

use cosmic::app::Core;
use cosmic::iced::{keyboard, Alignment, Fill, Subscription};
use cosmic::iced::event::{Event, Status};
use cosmic::widget::{column, container, row, scrollable, text, text_input, Space};
use cosmic::{Action, Application, Element, Task};

use serde::Deserialize;
use tokio::sync::mpsc;

use crate::focus_watcher;
use crate::shortcut_resolver::ShortcutResolver;

use crate::key_format::pretty_keys;

// ---------- JSON ----------
#[derive(Debug, Clone, Deserialize)]
struct ShortcutsFile {
    shortcuts: Vec<ShortcutEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct ShortcutEntry {
    keys: String,
    #[serde(alias = "description")]
    desc: String,
    #[serde(default)]
    category: Option<String>,
}

// ---------- Messages ----------
#[derive(Debug, Clone)]
pub enum Message {
    Toggle,
    AppIdChanged(String),
    SearchChanged(String),
    Tick,
}

// ---------- App ----------
pub struct OrbitKeysUi {
    core: Core,
    resolver: ShortcutResolver,

    app_id_text: String,
    search: String,

    // (keys, desc, category)
    items: Vec<(String, String, String)>,
    load_error: Option<String>,

    focus_rx: mpsc::UnboundedReceiver<String>,
    last_target_app_id: Option<String>,
}


/// Single-line truncation with ellipsis.
fn ellipsize(s: &str, max_chars: usize) -> String {
    let s = s.trim();
    if max_chars == 0 {
        return String::new();
    }
    let count = s.chars().count();
    if count <= max_chars {
        return s.to_string();
    }

    let keep = max_chars.saturating_sub(1);
    let mut out = String::with_capacity(max_chars);
    for (i, ch) in s.chars().enumerate() {
        if i >= keep {
            break;
        }
        out.push(ch);
    }
    out.push('…');
    out
}

/// Prevent wrapping by replacing spaces with NBSP.
/// This keeps it visually single-line inside narrow columns (it will clip/truncate instead of wrap).
fn no_wrap_spaces(s: &str) -> String {
    s.replace(' ', "\u{00A0}")
}

impl OrbitKeysUi {
    fn filtered_items(&self) -> Vec<(String, String, String)> {
        let q = self.search.trim().to_lowercase();
        if q.is_empty() {
            return self.items.clone();
        }

        self.items
            .iter()
            .cloned()
            .filter(|(k, d, c)| {
                k.to_lowercase().contains(&q)
                    || d.to_lowercase().contains(&q)
                    || c.to_lowercase().contains(&q)
            })
            .collect()
    }

    fn grouped_items(&self) -> BTreeMap<String, Vec<(String, String)>> {
        let mut map: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
        for (keys, desc, cat) in self.filtered_items() {
            map.entry(cat).or_default().push((keys, desc));
        }
        map
    }

    fn grouped_columns(&self, max_cols: usize) -> Vec<Vec<(String, Vec<(String, String)>)>> {
        let grouped = self.grouped_items();
        let mut cols: Vec<Vec<(String, Vec<(String, String)>)>> =
            (0..max_cols).map(|_| Vec::new()).collect();

        for (i, (category, entries)) in grouped.into_iter().enumerate() {
            cols[i % max_cols].push((category, entries));
        }

        cols
    }

    fn load_for_app_id(&mut self, app_id: &str) {
        self.items.clear();
        self.load_error = None;

        let app_id = app_id.trim();
        if app_id.is_empty() {
            return;
        }

        let Some(path) = self.resolver.resolve(app_id) else {
            self.load_error = Some(format!("No shortcuts for app_id: {app_id}"));
            return;
        };

        let raw = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                self.load_error = Some(e.to_string());
                return;
            }
        };

        let parsed: ShortcutsFile = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                self.load_error = Some(e.to_string());
                return;
            }
        };

        self.items = parsed
            .shortcuts
            .into_iter()
            .map(|s| {
                let cat = s.category.unwrap_or_else(|| "General".into());
                (s.keys, s.desc, cat)
            })
            .collect();
    }

    fn drain_focus_updates(&mut self) {
        let mut latest = None;
        while let Ok(v) = self.focus_rx.try_recv() {
            latest = Some(v);
        }

        let Some(app_id) = latest else { return };

        if app_id == Self::APP_ID {
            return;
        }

        let app_id = app_id.trim().to_string();
        if app_id.is_empty() || app_id == "unknown" {
            return;
        }

        if self.last_target_app_id.as_deref() == Some(&app_id) {
            return;
        }

        self.last_target_app_id = Some(app_id.clone());
        self.app_id_text = app_id.clone();
        self.load_for_app_id(&app_id);
    }
}

impl Application for OrbitKeysUi {
    const APP_ID: &'static str = "com.fonzi.orbitkeys";

    type Executor = cosmic::executor::Default;
    type Flags = ShortcutResolver;
    type Message = Message;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, resolver: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        let (tx, rx) = mpsc::unbounded_channel::<String>();

        tokio::task::spawn_blocking(move || {
            let _ = focus_watcher::run_focus_watcher(move |app_id| {
                let _ = tx.send(app_id);
            });
        });

        (
            Self {
                core,
                resolver,
                app_id_text: String::new(),
                search: String::new(),
                items: Vec::new(),
                load_error: None,
                focus_rx: rx,
                last_target_app_id: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::Toggle => {}
            Message::AppIdChanged(v) => {
                self.app_id_text = v;
                let id = self.app_id_text.trim().to_string();
                if !id.is_empty() {
                    self.last_target_app_id = Some(id.clone());
                    self.load_for_app_id(&id);
                }
            }
            Message::SearchChanged(v) => self.search = v,
            Message::Tick => self.drain_focus_updates(),
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let hotkey = cosmic::iced::event::listen_with(|event, _, _| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                if modifiers.logo()
                    && modifiers.shift()
                    && matches!(
                        key,
                        keyboard::Key::Character(ref c) if c.eq_ignore_ascii_case("s")
                    )
                {
                    Some((Status::Captured, Message::Toggle))
                } else {
                    None
                }
            }
            _ => None,
        })
        .map(|(_, msg)| msg);

        let tick = cosmic::iced::time::every(Duration::from_millis(90)).map(|_| Message::Tick);

        Subscription::batch(vec![hotkey, tick])
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let header = row()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(text("OrbitKeys").size(26))
            .push(Space::with_width(12))
            .push(text("App ID:").size(13))
            .push(
                text_input("focused app id…", &self.app_id_text)
                    .on_input(Message::AppIdChanged)
                    .width(220),
            );

        let search_row = row()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(text("Search:").size(13))
            .push(
                text_input("type to filter…", &self.search)
                    .on_input(Message::SearchChanged)
                    .width(Fill),
            );

        if self.items.is_empty() && self.load_error.is_none() {
            return container(
                column()
                    .spacing(16)
                    .width(Fill)
                    .height(Fill)
                    .push(header)
                    .push(search_row)
                    .push(
                        container(text("Focus an app to load shortcuts.").size(14))
                            .padding(16)
                            .width(Fill)
                            .height(Fill),
                    ),
            )
            .padding(16)
            .width(Fill)
            .height(Fill)
            .into();
        }

        let cols = self.grouped_columns(5);

        // Key bigger, desc directly UNDER it, single-line (truncate + NBSP to prevent wrapping).
        // Bold: libcosmic/iced text weight API varies by version; size + glyphs gets most of the effect.
        let key_size = 16;
        let desc_size = 12;
        let desc_max_chars = 22; // tune this as needed (smaller = less chance of overflow)
        let entry_gap = 6;

        let mut grid = row()
            .spacing(18)
            .width(Fill)
            .align_y(Alignment::Start);

        for col in cols {
            let mut col_widget = column().spacing(10).width(Fill);

            for (category, entries) in col {
                let mut cat_block = column()
                    .spacing(entry_gap)
                    .push(text(category).size(18));

                for (keys, desc) in entries {
                    let keys_pretty = pretty_keys(&keys);

                    // Force a single visual line:
                    // 1) remove newlines
                    // 2) ellipsize
                    // 3) replace spaces with NBSP so it won't wrap
                    let desc_one = no_wrap_spaces(&ellipsize(&desc.replace('\n', " "), desc_max_chars));


                    let entry = row()
                        .spacing(8)
                        .align_y(Alignment::Center)
                        .push(text(keys_pretty).size(key_size))
                        .push(text(desc_one).size(desc_size));

                    cat_block = cat_block.push(entry);

                }

                col_widget = col_widget.push(container(cat_block).padding(6));
            }

            grid = grid.push(col_widget);
        }

        let body: Element<'_, Self::Message> = if let Some(err) = &self.load_error {
            container(text(err).size(14))
                .padding(16)
                .width(Fill)
                .height(Fill)
                .into()
        } else {
            scrollable(container(grid).width(Fill))
                .height(Fill)
                .into()
        };

        container(
            column()
                .spacing(14)
                .width(Fill)
                .height(Fill)
                .push(header)
                .push(search_row)
                .push(body),
        )
        .padding(16)
        .width(Fill)
        .height(Fill)
        .into()
    }
}

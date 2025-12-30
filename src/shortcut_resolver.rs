use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};

#[derive(Clone, Debug)]
pub struct ShortcutResolver {
    shortcuts_dir: PathBuf,
    index: HashMap<String, PathBuf>,
}

impl ShortcutResolver {
    pub fn new(dir: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let mut s = Self {
            shortcuts_dir: dir.into(),
            index: HashMap::new(),
        };
        s.rebuild_index()?;
        Ok(s)
    }

    pub fn rebuild_index(&mut self) -> anyhow::Result<()> {
        self.index.clear();

        for entry in fs::read_dir(&self.shortcuts_dir)? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            self.index.insert(normalize(stem), path.clone());

            if let Ok(bytes) = fs::read(&path) {
                if let Ok(meta) = serde_json::from_slice::<Meta>(&bytes) {
                    if let Some(id) = meta.app_id {
                        self.index.insert(normalize(&id), path.clone());
                    }
                    for alias in meta.app_ids.unwrap_or_default() {
                        self.index.insert(normalize(&alias), path.clone());
                    }
                }
            }
        }

        Ok(())
    }

    pub fn resolve(&self, app_id: &str) -> Option<PathBuf> {
        let q = normalize(app_id);

        if let Some(p) = self.index.get(&q) {
            return Some(p.clone());
        }

        let mut keys: Vec<_> = self.index.keys().collect();
        keys.sort();

        for k in keys {
            if k.contains(&q) || q.contains(k) {
                return self.index.get(k).cloned();
            }
        }

        None
    }
}

#[derive(Deserialize)]
struct Meta {
    app_id: Option<String>,
    app_ids: Option<Vec<String>>,
}

fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use json::JsonValue;
use serde::{Deserialize, Serialize};

use crate::Yoink;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum KeyAction {
    Exclude,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ActionEntry {
    Action(KeyAction),
    Map(HashMap<String, ActionEntry>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonConfig {
    path: PathBuf,
    #[serde(default)]
    actions: HashMap<String, ActionEntry>,
    indent: Option<u16>,
}

impl Yoink for JsonConfig {
    fn pull(&mut self, _: Option<&[u8]>) -> anyhow::Result<Box<[u8]>> {
        let path = &self.path;

        // extract the json table from the main file
        let content = fs::read_to_string(path).map_err(path_err(path))?;
        let mut value = json::parse(&content).map_err(path_err(path))?;

        // modify the json according to the actions map
        modify_json(&mut value, &self.actions);

        // render the json and output it
        let spaces = self.indent.unwrap_or(4);
        Ok(value.pretty(spaces).into_bytes().into_boxed_slice())
    }
}

fn path_err<E: Display>(path: impl AsRef<Path>) -> impl FnOnce(E) -> anyhow::Error {
    move |err| anyhow!("'{}': {err}", path.as_ref().display())
}

fn modify_json(value: &mut JsonValue, actions: &HashMap<String, ActionEntry>) {
    let JsonValue::Object(object) = value else {
        return;
    };

    for (key, entry) in actions {
        let Some(value) = object.get_mut(key) else {
            continue;
        };

        match entry {
            ActionEntry::Action(KeyAction::Exclude) => {
                let _ = object.remove(key);
            }
            ActionEntry::Map(actions) => {
                modify_json(value, actions);
                if value.is_object() && value.is_empty() {
                    let _ = object.remove(key);
                }
            }
        }
    }
}

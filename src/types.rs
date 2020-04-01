use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// what the format should look like
pub struct TagFormat {
    pub object: String,
    pub tags: HashSet<String>,
}

impl fmt::Display for TagFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is: {:?}", self.object, self.tags)
    }
}

pub fn load(path: &Path) -> TagFormat {
    let content = fs::read_to_string(path).expect("Could not find file!");

    let tags: TagFormat = serde_json::from_str(&content).expect("Failed to deserialize");

    return tags;
}

pub fn save(tosave: &TagFormat, path: &Path) {
    let content = serde_json::to_string_pretty(tosave).expect("Failed to serialize");

    fs::write(path, &content).expect("Failed to write :(");
}

pub const TAG_FOLDER: &str = "tags";

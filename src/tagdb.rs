use std::collections::HashSet;
use std::path::Path;

use crate::types;
use crate::types::TagFormat;
use crate::GlobalOpts;
use crate::TagCommand;

pub(crate) fn tag(tagcmd: &TagCommand, globals: &GlobalOpts) {
    println!("{:?}, {:?}", tagcmd, globals);

    let filename = format!("{}/{}.json", types::TAG_FOLDER, tagcmd.object);
    let filepath = Path::new(&filename);
    let mut tags = match filepath.exists() {
        true => {
            println!("Loading already existing file");
            types::load(&filepath)
        }
        false => {
            println!("Creating new tag object");
            TagFormat {
                object: tagcmd.object.clone(),
                tags: HashSet::new(),
            }
        }
    };

    println!("Adding new tags: {:?}", tagcmd.tags);
    tags.tags.extend(tagcmd.tags.clone());

    types::save(&tags, &filepath);
}

use std::path::Path;

use crate::GlobalOpts;
use crate::TagCommand;

use crate::types;
use crate::types::TagFormat;

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
                tags: [].to_vec(),
            }
        }
    };

    let mut newtags = tagcmd.tags.clone();
    println!("Adding new tags: {:?}", newtags);
    tags.tags.append(&mut newtags);

    types::save(&tags, &filepath);
}

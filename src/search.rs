use std::path::Path;

use crate::types;
use crate::GlobalOpts;
use crate::SearchCommand;

pub(crate) fn search(searchcmd: &SearchCommand, globals: &GlobalOpts) {
    println!("{:?}, {:?}", searchcmd, globals);

    let filename = format!("{}/{}.json", types::TAG_FOLDER, searchcmd.query);
    let filepath = Path::new(&filename);
    let tags = types::load(&filepath);

    println!("{}", tags);
}

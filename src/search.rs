use crate::SearchCommand;
use crate::GlobalOpts;

pub(crate) fn search(searchcmd: &SearchCommand, globals: &GlobalOpts) {

    println!("{:?}, {:?}", searchcmd, globals);
}
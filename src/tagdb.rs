use crate::TagCommand;
use crate::GlobalOpts;

pub(crate) fn tag(tagcmd: &TagCommand, globals: &GlobalOpts) {

    println!("{:?}, {:?}", tagcmd, globals);
}
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// parse args
struct ArgVals {
    #[structopt(subcommand)]
    cmd: Command,

    #[structopt(short, long)]
    verbose: bool,
}

#[derive(StructOpt, Debug)]
/// list of commands
enum Command {
    Tag(TagCommand),
    Search(SearchCommand),
}

#[derive(StructOpt, Debug)]
/// set a new tag
struct TagCommand {
    /// item to tag
    object: String,

    /// tags to apply
    tags: Vec<String>,
}

#[derive(StructOpt, Debug)]
/// search through tag DB
struct SearchCommand {
    /// query string
    query: String,
}

fn main() {
    let av = ArgVals::from_args();

    match av.cmd {
        Command::Tag(asdf) => println!("going to tag {:?}", asdf),
        Command::Search(asdf) => println!("going to search {:?}", asdf),
    }

    println!("verbose {:?}", av.verbose);

}

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// parse args
struct ArgVals {
    /// some test thing
    #[argh(option)]
    test: i32,
}

fn main() {
    println!("Hello, world!");

    let av: ArgVals = argh::from_env();

    println!("{:?}", av);

}

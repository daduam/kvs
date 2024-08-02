use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string")
                .arg(arg!(<KEY>).required(true))
                .arg(arg!(<VALUE>).required(true)),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(arg!(<KEY>).required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a given key")
                .arg(arg!(<KEY>).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("get", _)) => panic!("unimplemented"),
        Some(("set", _)) => panic!("unimplemented"),
        Some(("rm", _)) => panic!("unimplemented"),
        _ => panic!(),
    }
}

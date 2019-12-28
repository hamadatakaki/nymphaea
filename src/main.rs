extern crate nymphaea;
use nymphaea::commands;

use clap::{App, Arg, SubCommand};

fn main() -> std::io::Result<()> {  // TODO: clap導入
    let add = SubCommand::with_name("add")
        .about("Staging file's changes");
    
    let cat_file = SubCommand::with_name("cat_file")
        .about("cat file");

    let init = SubCommand::with_name("init")
        .about("initialize repository");

    let commit = SubCommand::with_name("commit")
        .about("commit staging objects")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1)
        );

    let app = App::new("nymphaea")
        .version("0.1.0")
        .author("jellyfishrumble <i16hamadat@gmail.com>")
        .about("Test clap")
        .subcommands(
            vec![add, cat_file, init, commit]
        );

    let matches = app.get_matches();
    if let Some(_) = matches.subcommand_matches("init") {
        commands::init::init()?;
    }
    if let Some(_matches) = matches.subcommand_matches("add") {
        commands::add::add()?;
    }
    if let Some(_matches) = matches.subcommand_matches("commit") {
        println!("commit")
    }

    Ok(())
}

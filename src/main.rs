use clap::{Arg, ArgAction, Command};

mod config;
mod database;
mod generate;
mod libsql_driver;
mod migrate;

fn main() {
    let matches = Command::new("libsql-migeate")
        .about("Migrate tool for libsql")
        .version("1.0.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Emil Privér")
        .subcommands([
            Command::new("new").about("Create new migration").arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .help("Name for your new migration")
                    .action(ArgAction::Set)
                    .num_args(1),
            ),
            Command::new("up").about("Migrate to the latest version"),
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("new", query_matches)) => {
            let name = query_matches.get_one::<String>("name").unwrap();
            let new = generate::generate_new_migration(name);
            println!("{:?}", &new);
        }
        Some(("up", _)) => {
            match migrate::up() {
                Err(err) => {
                    println!("{:?}", err)
                }
                _ => {}
            };
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}

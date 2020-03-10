extern crate clap;

use clap::{App, Arg, SubCommand};
mod internals;

fn main() {
    let args = App::new("up")
        .version("1.0")
        .author("Sparkenstein")
        .about("nginx management made easy!")
        .subcommand(
            SubCommand::with_name("deploy")
                .about("generates all the files automatically and deploys the server")
                .arg(
                    Arg::with_name("servername")
                        .required(true)
                        .help("Name of the server. it will be used for `server` property in nginx"),
                )
                .arg(
                    Arg::with_name("port")
                        .default_value("80")
                        .short("p")
                        .long("port")
                        .help("Port at which this server listens"),
                )
                .arg(
                    Arg::with_name("webroot")
                        .default_value("/var/www/<servername>")
                        .short("w")
                        .long("webroot")
                        .help("root of the server."),
                ),
        )
        .get_matches();
    match args.subcommand() {
        ("deploy", Some(sub)) => internals::deploy::init(sub),
        _ => eprintln!("Please provide a command. use -h for help"),
    }
}

extern crate clap;

use clap::{App, Arg, SubCommand};
mod internals;

fn main() {
    let args = App::new("up")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sparkenstein")
        .about("nginx management made easy!")
        .subcommand(
            SubCommand::with_name("static")
                .about(" deploys the static server")
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
        .subcommand(
            SubCommand::with_name("proxy")
                .about("Sets up reverse proxy server")
                .arg(
                    Arg::with_name("servername")
                        .required(true)
                        .help("Name of the server. it will be used for `server` property in nginx"),
                )
                .arg(
                    Arg::with_name("inport")
                        .default_value("80")
                        .short("i")
                        .long("inport")
                        .help("Port at which this server listens"),
                )
                .arg(
                    Arg::with_name("outport")
                        .default_value("8080")
                        .short("o")
                        .long("outport")
                        .help("Reverse port at which server forwards request on localhost"),
                ),
        )
        .get_matches();
    match args.subcommand() {
        ("static", Some(sub)) => internals::statics::init(sub),
        ("proxy", Some(sub)) => internals::proxy::init(sub),
        _ => eprintln!("Please provide a command. use -h for help"),
    }
}

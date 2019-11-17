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
                .arg(Arg::with_name("servername").required(true)),
        )
        .get_matches();
    match args.subcommand_name() {
        Some("deploy") => internals::deploy::init(),
        _ => unreachable!()
    }
}

extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("up")
        .version("1.0")
        .author("Sparkenstein")
        .about("nginx management made easy!")
        .subcommand(
            SubCommand::with_name("deploy")
                .about("generates all the files automatically and deploys them")
                .arg(Arg::with_name("servername").takes_value(true)),
        )
        .get_matches();
    println!("{:?}", matches)
}

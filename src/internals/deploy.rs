use clap::ArgMatches;

pub fn init(value: &ArgMatches){
    println!("Deploying server with name {:?}", value.value_of("servername").unwrap());
}
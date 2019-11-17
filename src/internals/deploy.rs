use clap::ArgMatches;

pub fn init(value: &ArgMatches){
    // println!("Deploying server with name {:?}", value.args);
    let config: &str = include!("files/nginx.conf.sample");
    println!("{}", config.replace("${file}", "/etc/file"))
}
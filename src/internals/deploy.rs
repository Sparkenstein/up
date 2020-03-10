use clap::ArgMatches;

pub fn init(deploy_args: &ArgMatches) {
    let servername = deploy_args.value_of("servername").unwrap();
    let port = deploy_args.value_of("port").unwrap();

    let webroot;
    if deploy_args.occurrences_of("webroot") == 0 {
        webroot = deploy_args
            .value_of("webroot")
            .unwrap()
            .replace("<servername>", servername);
    } else {
        webroot = deploy_args.value_of("webroot").unwrap().to_string();
    }

    let config = include!("files/server.conf.sample")
        .replace("{{port}}", port)
        .replace("{{webroot}}", &webroot)
        .replace("{{domain}}", servername);
    println!("{}", config);
}

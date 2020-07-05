use crate::internals::utils;
use clap::ArgMatches;
use std::path::PathBuf;

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

    let server_config = include!("files/server.conf.sample")
        .replace("{{port}}", port)
        .replace("{{webroot}}", &webroot)
        .replace("{{domain}}", servername);

    let nginx_path = PathBuf::from(format!("/etc/nginx/sites-available/{}.conf", servername));

    // creating file later to avoid orphan files creating incase above code breaks.
    // nginx_path.push();
    if nginx_path.exists() {
        eprintln!(
            "Site with {} name already exists at sites-available/",
            servername
        );
        std::process::exit(1);
    }

    utils::write_file(&nginx_path, server_config);

    utils::make_symlink(nginx_path);
}

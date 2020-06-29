use crate::internals::utils;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn init(deploy_args: &ArgMatches) {
    let servername = deploy_args.value_of("servername").unwrap();
    let port = deploy_args.value_of("port").unwrap();
    let mut nginx_path = PathBuf::new();

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

    // creating file later to avoid orphan files creating incase above code breaks.
    nginx_path.push(format!("/etc/nginx/sites-available/{}.conf", servername));
    if nginx_path.exists() {
        eprintln!(
            "Site with {} name already exists at sites-available/",
            servername
        );
        std::process::exit(1);
    }
    utils::backup_config();

    utils::gen_dh_params();

    utils::write_file(&nginx_path, config);

    utils::make_symlink(nginx_path);
}

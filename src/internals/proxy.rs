use crate::internals::utils;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn init(deploy_args: &ArgMatches) {
    let servername = deploy_args.value_of("servername").unwrap();
    let inport = deploy_args.value_of("inport").unwrap();
    let outport = deploy_args.value_of("outport").unwrap();
    let mut nginx_path = PathBuf::new();

    let config = include!("files/proxy_server.conf.sample")
        .replace("{{inport}}", inport)
        .replace("{{outport}}", outport)
        .replace("{{domain}}", servername);

    nginx_path.push(format!("/etc/nginx/sites-available/{}.conf", servername));

    utils::write_file(&nginx_path, config);
    utils::make_symlink(nginx_path);
}

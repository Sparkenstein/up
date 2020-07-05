use crate::internals::utils;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn init(deploy_args: &ArgMatches) {
    let servername = deploy_args.value_of("servername").unwrap();
    let inport = deploy_args.value_of("inport").unwrap();
    let outport = deploy_args.value_of("outport").unwrap();

    let config = include!("files/proxy_server.conf.sample")
        .replace("{{inport}}", inport)
        .replace("{{outport}}", outport)
        .replace("{{domain}}", servername);

    let nginx_path = PathBuf::from(format!("/etc/nginx/sites-available/{}.conf", servername));

    utils::backup_config();

    utils::gen_dh_params();

    utils::write_file(&nginx_path, config);

    utils::make_symlink(nginx_path);
}

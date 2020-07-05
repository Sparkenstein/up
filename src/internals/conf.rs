use crate::internals::utils;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn init(deploy_args: &ArgMatches) {
    let dns_resolvers = deploy_args.value_of("dns").unwrap();

    let nginx_config = include!("files/nginx.conf.sample")
        .replace("{{max_client_body_size}}", "16M")
        .replace("{{dns_resolvers}}", dns_resolvers);

    utils::backup_config();

    utils::gen_dh_params();
    let file_path = PathBuf::from("/etc/nginx/nginx.conf");
    utils::write_file(&file_path, nginx_config);

    // Copy other config files now
    std::fs::create_dir_all("/etc/nginx/upconf/").unwrap();

    let file_path = PathBuf::from("/etc/nginx/upconf/general.conf");
    let general_conf = include!("files/upconf/general.conf.sample").to_string();
    utils::write_file(&file_path, general_conf);

    let file_path = PathBuf::from("/etc/nginx/upconf/letsencrypt.conf");
    let letsencrypt_conf = include!("files/upconf/letsencrypt.conf.sample").to_string();
    utils::write_file(&file_path, letsencrypt_conf);

    let file_path = PathBuf::from("/etc/nginx/upconf/proxy.conf");
    let proxy_conf = include!("files/upconf/proxy.conf.sample").to_string();
    utils::write_file(&file_path, proxy_conf);

    let file_path = PathBuf::from("/etc/nginx/upconf/security.conf.sample");
    let security_conf = include!("files/upconf/security.conf.sample").to_string();
    utils::write_file(&file_path, security_conf);
}

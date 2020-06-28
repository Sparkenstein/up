use clap::ArgMatches;
use std::fs::File;
use std::os::unix::fs;
use std::io::ErrorKind;
use std::io::Write;
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

    let mut file = File::create(&nginx_path).unwrap_or_else(|e| match e.kind() {
        ErrorKind::PermissionDenied => {
            eprintln!("Error occurred while creating config files, Permission Denied. Are you running as sudo?");
            std::process::exit(1)
        }
        _ => {
            std::process::exit(1)
        }
    });

    file.write_all(config.as_bytes())
        .unwrap_or_else(|e| eprintln!("Error writing file to path {}", e));
    let symlink_path = &nginx_path
        .to_str()
        .unwrap()
        .replace("sites-available", "sites-enabled");
    fs::symlink(nginx_path, symlink_path)
        .unwrap_or_else(|e| eprintln!("Error Symlinking file {}", e));
}

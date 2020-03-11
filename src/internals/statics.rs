use clap::ArgMatches;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
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

    let mut file = File::create(nginx_path).unwrap_or_else(|e| match e.kind() {
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
}

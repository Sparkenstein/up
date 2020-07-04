use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::os::unix::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn make_symlink(nginx_path: PathBuf) {
    let mut symlink_path = PathBuf::from("/etc/nginx/sites-enabled");
    symlink_path.push(nginx_path.file_name().unwrap());
    fs::symlink(nginx_path, symlink_path)
        .unwrap_or_else(|e| eprintln!("Error Symlinking file {}", e));
}

pub fn write_file(nginx_path: &PathBuf, config: String) {
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

pub fn backup_config() {
    println!("Creating backup of current configuration in /etc/nginx");

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let command = "tar";
    let op = Command::new(command)
        .arg("-czvf")
        .arg(format!("nginx_{}.tar.gz", since_the_epoch.as_millis()))
        .arg("nginx.conf")
        .arg("sites-available/")
        .arg("sites-enabled/")
        .current_dir("/etc/nginx/")
        .stdout(Stdio::null())
        .output()
        .expect("Failed to execute");
    if op.status.success() {
        println!("Backup created successfully");
    } else {
        eprintln!("Failed to create backup of current config");
        std::process::exit(1);
    }
}

pub fn gen_dh_params() {
    let current_file = PathBuf::from("/etc/nginx/dhparam.pem");
    if !current_file.exists() {
        println!("Generating DH parameters, 2048 bit long safe prime, generator 2");
        println!("This is going to take a LONG time");
        let dhparams = Command::new("openssl")
            .arg("dhparam")
            .arg("-out")
            .arg("/etc/nginx/dhparam.pem")
            .arg("128")
            // .arg("&>/dev/null")
            .stdout(Stdio::null())
            .output()
            .expect("Error in generating dhparams");
        if dhparams.status.success() {
            print!("dhparams generated succesfully\n")
        }
    }
}

pub fn copy_base_config(base_config: String) {
    let mut conf_file = File::create("/etc/nginx/nginx.conf").unwrap_or_else(|e| match e.kind() {
        ErrorKind::PermissionDenied => {
            eprintln!("Error occurred while creating config files, Permission Denied. Are you running as sudo?");
            std::process::exit(1)
        }
        ErrorKind::NotFound => {
            eprintln!("Cannot find the path specified");
            std::process::exit(1)
        }
        _ => {
            eprintln!("Error");
            std::process::exit(1)
        }
    });
    conf_file
        .write_all(base_config.as_bytes())
        .unwrap_or_else(|e| eprintln!("Error writing file to path {}", e));
}

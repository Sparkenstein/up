use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::os::unix::fs;
use std::path::PathBuf;
use std::process::Command;
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
    println!("\nCreating backup of current configuration in curent directory");

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
        .status()
        .expect("Failed to execute");
    if op.success() {
        println!("Backup created successfully");
    } else {
        eprintln!("Failed to create backup of current config");
        std::process::exit(1);
    }
}

use std::path::PathBuf;
use std::os::unix::fs;
use std::fs::File;
use std::io::Write;
use std::io::ErrorKind;

pub fn make_symlink(nginx_path: PathBuf){
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
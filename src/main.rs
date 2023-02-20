use std::env::args;
use std::fs;
use std::path::PathBuf;

fn print_help () {

    let help = [
        "",
        "============================================ Help information ==========================================",
        "",
        "clearlocalhost deletes data from the local host ( possibly http://127.0.0.1 ) in the Chrome web browser.",
        "This can be useful when a webpage cannot be deleted from the local host.",
        "",
        "Usage:",
        "",
        "   \"./clearlocalhost\"       : Deletes data from \"path1\"",
        "",
        "   \"./clearlocalhost all\"   : Deletes data from \"path1\", \"path2\", \"path3\", \"path4\", and \"path5\"",
        "",
        "   \"./clearlocalhost help\"  : Print help information",
        "",
        "Try running \"clearlocalhost\" to clear only \"path1\". If that does not work, run \"clearlocalhost all\".",
        "",
        "========================================================================================================",
        "",
    ];

    for line in help {
        println!("{}", line);
    }
}

fn clear_path (path: PathBuf) {

    println!("");
    println!(
        "========================== Deleting cache data from the local host. Please wait. =======================",
    );
    println!("path: {:?}", path);

    match fs::remove_dir_all(&path) {
        Ok(_) => {
            println!("Cache files have been removed successfully!");
        },
        Err(e) => {
            println!("Failed to remove cache files: {}", e);
        }
    }
}

fn main() {

    let path1: PathBuf = 
        ["C:\\", "Users", "user", "AppData", "Local", "Google", "Chrome", "User Data", "Default", "Service Worker", "CacheStorage"]
        .iter()
        .collect();
    let path2: PathBuf = 
        ["C:\\", "Users", "user", "AppData", "Local", "Google", "Chrome", "User Data", "Default", "Local Storage", "leveldb"]
        .iter()
        .collect();
    let path3: PathBuf = 
        ["C:\\", "Users", "user", "AppData", "Local", "Google", "Chrome", "User Data", "Default", "IndexedDB", "http_127.0.0.1_8080.indexeddb.leveldb"]
        .iter()
        .collect();
    let path4: PathBuf = 
        ["C:\\", "Users", "user", "AppData", "Local", "Google", "Chrome", "User Data", "Default", "Cache", "Cache_Data"]
        .iter()
        .collect();
    let path5: PathBuf = 
        ["C:\\", "Users", "user", "AppData", "Local", "Google", "Chrome", "User Data", "Default", "Code Cache", "js"]
        .iter()
        .collect();

    let arg1 = args().nth(1);

    println!("");
    println!(
        "============================================     Path List    ==========================================",
    );
    println!("");
    println!("path1 : {:?}", path1);
    println!("path2 : {:?}", path2);
    println!("path3 : {:?}", path3);
    println!("path4 : {:?}", path4);
    println!("path5 : {:?}", path5);

    match arg1.as_deref() {

        None => {
            print_help();
            clear_path(path1);
            }
        Some("all") => {
            clear_path(path1);
            clear_path(path2);
            clear_path(path3);
            clear_path(path4);
            clear_path(path5);
            }
        Some("help") => {
            print_help();
            }
        _ => {
            print_help();
            }

    }

}

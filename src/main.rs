use std::env;
use std::env::args;
use std::fs;
use std::path::PathBuf;

fn print_help() {
    
    let help = r#"
============================================ Help information ==========================================

ClearLocalHost v1.2.0

clearlocalhost deletes data from the local host ( possibly http://127.0.0.1 ) in the Chrome web browser.
This can be useful when a webpage cannot be deleted from the local host.

Usage:

   "./clearlocalhost"        : Deletes data from "path1" in Google Chrome

   "./clearlocalhost all"    : Deletes data from "path1", "path2", "path3", "path4", and "path5"

   "./clearlocalhost help"   : Print help information

   "./clearlocalhost chrome" : Deletes data in Google Chrome ( = default )

   "./clearlocalhost edge"   : Deletes data in Microsoft Edge

   "./clearlocalhost whale"  : Deletes data in Naver Whale

Try running "clearlocalhost" to clear only "path1". If that does not work, run "clearlocalhost all".

========================================================================================================
"#;

    for line in help.lines() {
        println!("{}", line);
    }
}

fn make_fullpath(path : PathBuf) -> PathBuf {

    let appdata_path = env::var("LOCALAPPDATA").unwrap();
    let mut fullpath = PathBuf::from(appdata_path);

    fullpath.push(path);
    fullpath
}

fn clear_path (path: PathBuf) {

    println!("");
    println!(
        "========================== Deleting cache data from the local host. Please wait. =======================",
    );
    println!("remove path: {:?}", path);

    match fs::remove_dir_all(&path) {
        Ok(_) => {
            println!("Cache files have been removed successfully!");
        },
        Err(e) => {
            println!("Failed to remove cache files: {}", e);
        }
    }
    println!("");
}

fn main() {

    let mut path1: PathBuf = 
        ["Google", "Chrome", "User Data", "Default", "Service Worker", "CacheStorage"]
        .iter()
        .collect();
    let mut path2: PathBuf = 
        ["Google", "Chrome", "User Data", "Default", "Local Storage", "leveldb"]
        .iter()
        .collect();
    let mut path3: PathBuf = 
        ["Google", "Chrome", "User Data", "Default", "IndexedDB", "http_127.0.0.1_8080.indexeddb.leveldb"]
        .iter()
        .collect();
    let mut path4: PathBuf = 
        ["Google", "Chrome", "User Data", "Default", "Cache", "Cache_Data"]
        .iter()
        .collect();
    let mut path5: PathBuf = 
        ["Google", "Chrome", "User Data", "Default", "Code Cache", "js"]
        .iter()
        .collect();

    path1 = make_fullpath (path1);
    path2 = make_fullpath (path2);
    path3 = make_fullpath (path3);
    path4 = make_fullpath (path4);
    path5 = make_fullpath (path5);
    println!("");

    let arg1 = args().nth(1);

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
        Some("chrome") => {
            let mut path1: PathBuf = 
                ["Google", "Chrome", "User Data", "Default", "Service Worker", "CacheStorage"]
                .iter()
                .collect();
            path1 = make_fullpath (path1);
            clear_path(path1);
        }
        Some("edge") => {
            let mut path1: PathBuf = 
                ["Microsoft", "Edge", "User Data", "Default", "Service Worker", "CacheStorage"]
                .iter()
                .collect();
            path1 = make_fullpath (path1);
            clear_path(path1);
        }
        Some("whale") => {
            let mut path1: PathBuf = 
                ["Naver", "Naver Whale", "User Data", "Default", "Service Worker", "CacheStorage"]
                .iter()
                .collect();
            path1 = make_fullpath (path1);
            clear_path(path1);
        }
        Some("help") => {
            print_help();
        }
        _ => {
            print_help();
        }

    }

}

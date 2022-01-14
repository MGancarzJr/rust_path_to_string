use std::env;
use std::path::Path;
use std::ffi::OsString;

fn main() {
    let args: Vec<String> = env::args().collect();

    let full_path = Path::new(&args[0]);

    let mut file_name = String::new();
    let mut file_parent = String::new();

    // Extract the parent string in the path using method 1
    let _ = match full_path.parent() {
        Some(result) => {
            let _ = match result.to_str() {
                Some(value) => file_parent.clone_from(&value.to_string()),
                None => (),
            };
        },
        None => (),
    };

    // Extract the file name string in the path using method 1
    let _ = match full_path.file_stem() {
        Some(result) => {
            let _ = match result.to_str() {
                Some(value) => file_name.clone_from(&value.to_string()),
                None => (),
            };
        },
        None => (),
    };

    // Extract the extension string in the path using method 2
    // Here, value is of type &OsStr, but this code constructs and actual OsString object from the reference
    // It's the only way to make both arms of the match return an identical Type to some_value
    let some_value = match full_path.extension() {
        Some(value) => OsString::from(value),
        None => OsString::new(),
    };

    let file_ext = match some_value.to_str() {
        Some(content) => String::from(content),
        None => String::new(),
    };

    println!("Executable parent: {}", file_parent);
    println!("Executable stem: {}", file_name);
    println!("Executable extension: {}", file_ext);
}
use std::env;
use std::path::Path;

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
    let some_value = match full_path.extension() {
        Some(value) => value,
        None => panic!(),
    };

    // Will panic when compiled on a Unix/Linux machine or if extension is removed from executable name
    let file_ext = match some_value.to_str() {
        Some(content) => String::from(content),
        None => panic!(),
    };

    println!("Executable parent: {}", file_parent);
    println!("Executable stem: {}", file_name);
    println!("Executable extension: {}", file_ext);
}

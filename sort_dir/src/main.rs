use std::env::args;
// use std::fs;
use std::path::Path;

// fn main() {
//     let _args = args().collect::<Vec<String>>();
//     let dir: String = _args[1].clone();
//     let dir_path = Path::new(&dir);
//     println!("Sorting files in {}...", dir);
// }

// fn get_file_extension(file_name: String) -> String {
//     let mut file_extension: String = String::new();
//     let mut file_name_chars = file_name.chars();
//     let mut char = file_name_chars.next();
//     while char != None {
//         if char == Some('.') {
//             file_extension = String::new();
//         } else {
//             file_extension.push(char.unwrap());
//         }
//         char = file_name_chars.next();
//     }
//     file_extension
// }

// fn get_files(dir: &Path) -> Vec<String> {
//     let mut files = Vec::new();
//     let dir_entries = fs::read_dir(dir);
//     match dir_entries {
//         Ok(entries) => {
//             for entry in entries {
//                 let entry = entry.unwrap();
//                 let file_name = entry.file_name().into_string().unwrap();
//                 let file_extension = get_file_extension(file_name);
//                 if file_extension != "" {
//                     files.push(file_name);
//                 }
//             }
//         }
//         Err(_) => {
//             println!("Error reading directory entries.");
//         }
//     }

//     files
// }

// fn check_for_dir(file_ext: String, dir: String) -> std::path::PathBuf {
//     let mut dir_path = std::path::PathBuf::new();
//     if fs::join(dir, file_ext).is_dir() {
//         // println!("{} is a directory.", file_ext);
//     } else {
//         //print creating directory: dir path, sep, file_ext
//         println!("Creating directory: {}{}{}", dir, std::path::MAIN_SEPARATOR, file_ext);
//         // Create directory
//         fs::create_dir(fs::join(dir, file_ext)).unwrap();
//     }
// }

fn main() {
    let args: Vec<String> = args().collect();
    let relative_path = &args[1];
    let absolute_path = Path::new(relative_path).canonicalize();
    match absolute_path {
        Ok(path) => {
            println!("Path: {}", path.display());
            // files = list_files(path)
        }
        Err(_) => {
            println!("Path: {} could mot be resolved.", relative_path);
        }
    }
}

/*
// this function will:
// 1. take a path argument, and iterate through the files in the
//    directory, ignoring directories
// 2. fill the vector with the file's absolute path and extension
//    via calling the get_file_extension function
*/
// fn list_files(path: &Path) -> Vec<(&Path, &String)> {}

/*
this function will:
1. take a Path argument and parse the file name to get the extension
2. return the extension as a String
*/
// fn get_file_extension(file_path: &Path) -> String {}

/*
this function will:
iterate through the vector of tuples: (Path, String) and:
    1. check if a directory corresponding to the file extension exists,
       if not, create it using the check_for_dir function.
       (check_for_dir will return the path to the directory)
    2. move the file to the directory corresponding to the file extension
*/
// fn sort_files(root: &Path, files: Vec<(&Path, &String)>) {}

/*
this function will:
1. take a tuple of (Path, String) and check if a directory exists
   corresponding to the file extension, if not, create it.
2. return the path to the directory
*/
// fn check_for_dir(file_tup: (Path, String)) -> Path {}

use std::env::args;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let relative_path = &args[1];
    let absolute_path = Path::new(relative_path).canonicalize();
    match absolute_path {
        Ok(path) => {
            // "sorting <num files> in <path>"
            println!(
                "Sorting {} files in {}",
                list_files(path.to_str().unwrap().to_string()).len(),
                path.to_str().unwrap()
            );
            let root_dir: String = path.to_str().unwrap().to_string();
            let files = list_files(path.to_str().unwrap().to_string());
            for file in files {
                if is_audio(&file.1) {
                    sort_file(file.0, &root_dir, "audio".to_string());
                } else if is_video(&file.1) {
                    sort_file(file.0, &root_dir, "video".to_string());
                } else if is_document(&file.1) {
                    sort_file(file.0, &root_dir, "document".to_string());
                } else if is_image(&file.1) {
                    sort_file(file.0, &root_dir, "image".to_string());
                } else if is_executable(&file.1) {
                    sort_file(file.0, &root_dir, "executable".to_string());
                } else if is_compressed(&file.1) {
                    sort_file(file.0, &root_dir, "compressed".to_string());
                } else if is_script_or_code(&file.1) {
                    sort_file(file.0, &root_dir, "code".to_string());
                } else {
                    sort_file(file.0, &root_dir, "misc".to_string());
                }
            }
            println!("Done! All files are sorted.");
        }
        Err(_) => {
            println!("Path: {} could mot be resolved.", relative_path);
        }
    }
}

fn list_files(path: String) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_extension = get_file_extension(file_name);
            // files.push((path, file_extension));
            files.push((path.to_str().unwrap().to_string(), file_extension));
        }
    }
    files
}

fn get_file_extension(file_name: &str) -> String {
    let file_name = file_name.to_string();
    let file_extension: Vec<&str> = file_name.split('.').collect();
    let file_extension = file_extension[file_extension.len() - 1];
    file_extension.to_string()
}

fn is_audio(file_extension: &String) -> bool {
    let audio_extensions = vec!["mp3", "wav", "flac", "m4a", "aac", "ogg"];
    let is_audio = audio_extensions.contains(&file_extension.as_str());
    is_audio
}
fn is_video(file_extension: &String) -> bool {
    let video_extensions = vec!["mp4", "mkv", "avi", "mov", "wmv", "flv"];
    let is_video = video_extensions.contains(&file_extension.as_str());
    is_video
}
fn is_document(file_extension: &String) -> bool {
    let document_extensions = vec![
        "doc", "docx", "xls", "xlsx", "ppt", "pptx", "pdf", "txt", "md", "html", "css", "js",
    ];
    let is_document = document_extensions.contains(&file_extension.as_str());
    is_document
}

fn is_image(file_extension: &String) -> bool {
    let image_extensions = vec!["jpg", "jpeg", "png", "gif", "tiff", "psd", "bmp", "ico"];
    let is_image = image_extensions.contains(&file_extension.as_str());
    is_image
}

fn is_executable(file_extension: &String) -> bool {
    let executable_extensions = vec!["exe", "msi", "deb", "rpm"];
    let is_executable = executable_extensions.contains(&file_extension.as_str());
    is_executable
}

fn is_compressed(file_extension: &String) -> bool {
    let compressed_extensions = vec!["zip", "rar", "tar", "gz", "dmg"];
    let is_compressed = compressed_extensions.contains(&file_extension.as_str());
    is_compressed
}

fn is_script_or_code(file_extension: &String) -> bool {
    let script_or_code_extensions = vec!["sh", "rs", "py", "js", "html", "css", "ts", "go", "rb"];
    let is_script_or_code = script_or_code_extensions.contains(&file_extension.as_str());
    is_script_or_code
}

fn check_create_dir(folder_name: &String, root_dir: &String) {
    let path = Path::new(&root_dir).join(folder_name);
    if !path.exists() {
        fs::create_dir(path).unwrap();
        println!("Created directory: {}", folder_name);
    }
}

fn sort_file(file_path: String, root_dir: &String, file_type: String) {
    check_create_dir(&file_type, root_dir);
    let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
    let new_path = Path::new(&root_dir).join(file_type).join(file_name);
    fs::rename(file_path, new_path).unwrap();
}

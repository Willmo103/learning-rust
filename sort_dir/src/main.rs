use std::env::args;
// use std::fs;
use std::path::Path;

struct FileSorter<'a> {
    documents: Vec<&'a str>,
    images: Vec<&'a str>,
    audio: Vec<&'a str>,
    video: Vec<&'a str>,
    compressed: Vec<&'a str>,
    executables: Vec<&'a str>,
    code: Vec<&'a str>,
    root: &'a Path,
    folder_names: Vec<&'a str>,
}

impl<'a> FileSorter<'a> {
    fn new(root: &'a Path) -> Self {
        let documents = vec![
            ".pdf", ".doc", ".docx", ".txt", ".rtf", ".odt", ".pages", ".epub", ".md", ".csv",
            ".xls", ".xlsx", ".ppt", ".pptx", ".odp", ".numbers", ".key",
        ];

        let images = vec![
            ".jpg", ".jpeg", ".png", ".gif", ".svg", ".bmp", ".tiff", ".tif", ".psd", ".ai", ".raw",
        ];

        let audio = vec![
            ".mp3", ".wav", ".aiff", ".aif", ".aifc", ".flac", ".m4a", ".wma", ".aac", ".pcm",
            ".ogg", ".oga", ".opus", ".webm",
        ];

        let video = vec![
            ".mp4", ".mov", ".avi", ".wmv", ".mpg", ".mpeg", ".mkv", ".flv", ".3gp", ".3g2",
            ".m4v", ".webm", ".ogv",
        ];

        let compressed = vec![
            ".zip", ".rar", ".7z", ".gz", ".bz2", ".tar", ".iso", ".dmg", ".pkg", ".deb", ".rpm",
        ];

        let executables = vec![
            ".exe", ".msi", ".bin", ".sh", ".bat", ".jar", ".app", ".apk", ".dmg", ".pkg", ".deb",
            ".rpm",
        ];

        let code = vec![
            ".html", ".css", ".js", ".ts", ".jsx", ".tsx", ".php", ".py", ".rb", ".java", ".c",
            ".cpp", ".h", ".cs", ".go", ".swift", ".rs", ".pl", ".sql", ".xml", ".json", ".yml",
            ".yaml", ".toml", ".md",
        ];
        let root: &Path = root;
        let folder_names = vec![
            "Documents",
            "Images",
            "Audio",
            "Video",
            "Compressed",
            "Executables",
            "Code",
        ];

        FileSorter {
            documents,
            images,
            audio,
            video,
            compressed,
            executables,
            code,
            root,
            folder_names,
        }
    }

    fn check_dir(&self, file_tup: (&Path, &str)) -> &Path {
        let file_path = file_tup.0;
        let file_ext = file_tup.1;
        let mut dir_path = self.root.to_path_buf();
        let mut dir_name = String::new();
        if self.documents.contains(&file_ext) {
            dir_name = self.folder_names[0].to_string();
        } else if self.images.contains(&file_ext) {
            dir_name = self.folder_names[1].to_string();
        } else if self.audio.contains(&file_ext) {
            dir_name = self.folder_names[2].to_string();
        } else if self.video.contains(&file_ext) {
            dir_name = self.folder_names[3].to_string();
        } else if self.compressed.contains(&file_ext) {
            dir_name = self.folder_names[4].to_string();
        } else if self.executables.contains(&file_ext) {
            dir_name = self.folder_names[5].to_string();
        } else if self.code.contains(&file_ext) {
            dir_name = self.folder_names[6].to_string();
        } else {
            dir_name = "Other".to_string();
        }
        dir_path.push(dir_name);
        if !dir_path.exists() {
            fs::create_dir(&dir_path).expect("Could not create directory");
        }
        dir_path.as_path()
    }

    fn sort_files(&self, files: Vec<(&Path, &str)>) {
        for file in files {
            let dir_path = self.check_dir(file);
            fs::rename(file.0, dir_path.join(file.0.file_name().unwrap()))
                .expect("Could not move file");
        }
    }
}

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
fn list_files(path: &Path) -> Vec<(&Path, &String)> {
    let mut files: Vec<(&Path, &String)> = Vec::new();
    for entry in fs::read_dir(path).expect("Could not read directory") {
        let entry = entry.expect("Could not get entry");
        let file_path = entry.path();
        if file_path.is_file() {
            let file_ext = get_file_extension(&file_path);
            files.push((&file_path, &file_ext));
        }
    }
    files
}

/*
this function will:
1. take a Path argument and parse the file name to get the extension
2. return the extension as a String
*/
fn get_file_extension(file_path: &Path) -> String {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_name_split: Vec<&str> = file_name.split('.').collect();
    let file_ext = file_name_split[file_name_split.len() - 1];
    file_ext.to_string()
}

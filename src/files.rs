extern crate std;
extern crate ructe;
use std::fs;

pub fn load_files_in_directory(directory_path: &String) -> Vec<File> {
    let files = fs::read_dir(&directory_path).unwrap();
    return files.map(|f| load_file(f.unwrap())).collect::<Vec<File>>();
}

fn load_file(file: fs::DirEntry) -> File {
    let p = file.path().to_str().unwrap().to_string(); // wow
    let ext = p.clone().rsplit('.').next().unwrap().to_string();
    let mut html_name = file.file_name().into_string().unwrap();
    html_name.push_str(".html");


    File {
        name: file.file_name().to_str().unwrap().to_string(), // wow
        path: p,
        html_name: html_name,
        extension: ext,
        file_type: file.file_type().unwrap()
    }
}

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub html_name: String,
    pub extension: String,
    pub file_type: fs::FileType
}
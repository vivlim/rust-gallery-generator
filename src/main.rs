mod files;
mod handlers;

use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::{Path};
use handlers::RenderedFile;

fn main() {
    println!("Hello, world!");
    let f = files::load_files_in_directory(&"./input".to_string());

    let output_dir = Path::new("./output");
    create_dir_all(output_dir);

    let handled_files: Vec<RenderedFile> = f.iter().map(|f| handlers::render_file(f)).collect();

    let mut out_file = File::create(output_dir.join("index.html")).unwrap();
    templates::index(&mut out_file, "input", "output", &handled_files);

    for handled_file in handled_files {
        println!("{}: {}", handled_file.file.path, handled_file.output);
        let mut out_file = File::create(output_dir.join(handled_file.file.html_name)).unwrap();
        out_file.write_all(handled_file.output.as_bytes());
    }
}

// And finally, include the generated code for templates and static files.
include!(concat!(env!("OUT_DIR"), "/templates.rs"));

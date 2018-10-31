use files::File;
use std::io::{self, Read, Write, Cursor};
use templates;

pub struct RenderedFile {
    pub file: File,
    pub output: String
}

pub fn render_file(file: &File) -> RenderedFile {
    let mut output = Cursor::new(Vec::new());

    let result = match file.extension.as_ref() {
        "swf" => templates::swf(&mut output, &file),
        _ => templates::fallback(&mut output, &file)
    };

    let mut output_string = String::new();
    output.read_to_string(&mut output_string);

    RenderedFile {
        file: file.clone(),
        output: String::from_utf8(output.into_inner()).unwrap()
    }
}
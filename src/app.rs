use std::path::PathBuf;
use walkdir::WalkDir;
use fs_extra::dir::get_size;
use crate::{cli::Language, common::format_bytes};

#[derive(Default, Debug)]
pub struct File {
    path: PathBuf,
    size: u64,
}

impl File {
    pub fn new(path: PathBuf, size: u64) -> Self {
        Self { path, size }
    }
}

#[derive(Default, Debug)]
pub struct App {
    pub path: PathBuf,
    pub lang: Language,
    pub files: Vec<File>,
}

impl App {
    // Todo: 需要重构适配不同的语言缓存
    pub fn scan(&mut self) {
        println!("Please wait while scanning directories...");
        for entry in WalkDir::new(&self.path) {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let cur_path = entry.path();
                        let is_existing = &self.files.iter().find(|&file| cur_path.starts_with(&file.path));
                        if !is_existing.is_some() {
                            let file_name = cur_path.file_name().unwrap_or_default();
                            if self.lang == Language::Node && file_name == "node_modules" {
                                self.files.push(File {
                                    path: entry.path().into(),
                                    size: get_size(cur_path).unwrap()
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub fn del(&self) {
        println!("Please wait while deleting the directory...");
        for file in self.files.iter() {
            println!("del: {:?}", file);
        }
    }

    pub fn show_info(&self) -> String {
        let mut str: String = String::new();
        for file in self.files.iter() {
            str.push_str(&format!("dels: {}, size: {} \r\n", file.path.display(), format_bytes(file.size)));
        }

        str.push_str(&format!("del files count: {}\r\n", self.files.len()));
        
        println!("{}", str);

        str
    }
}

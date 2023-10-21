use lazy_static::lazy_static;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref PROGRAM_DIR: PathBuf = current_dir().unwrap();
    pub static ref SIGN_API_DIR: PathBuf = PROGRAM_DIR.join("API");
    pub static ref YUNZAI_DIR: PathBuf = {
        let mut default_name = "Yunzai-bot".to_string();
        fs::read_dir(PROGRAM_DIR.as_path()).unwrap().for_each(|dir| {
            let dir_name = dir.unwrap().file_name().into_string().unwrap();
            if dir_name.contains("Miao-Yunzai") {
                default_name = dir_name;
            }
        });
        PROGRAM_DIR.join(Path::new(default_name.as_str()))
    };
}

use std::fs::read_dir;

use crate::file_utils::clean_scenes::clean_scene;

mod clean_scenes;

pub fn clean_scenes() {
    for path in read_dir("scenes").unwrap() {
        let path = path.unwrap().path();
        let file = std::fs::read(&path).unwrap();
        if file.is_empty() {
            continue;
        }
        let cleaned = clean_scene(file.into_iter().map(char::from).collect());
        std::fs::write(path, cleaned).unwrap();
    }
}

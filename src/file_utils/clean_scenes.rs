use std::fs::read_dir;

pub fn clean_scenes() {
    for path in read_dir("scenes").unwrap() {
        let path = path.unwrap().path();
        let file = std::fs::read(&path).unwrap();
        let lowered = file
            .into_iter()
            .map(|char| (char as char).to_ascii_lowercase())
            .collect::<String>();
        std::fs::write(path, lowered).unwrap();
    }
}

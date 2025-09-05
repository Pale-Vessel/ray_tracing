use std::fs::read_dir;

pub fn clean_scenes() {
    for path in read_dir("scenes").unwrap() {
        let path = path.unwrap().path();
        let file = std::fs::read(&path).unwrap();
        let lowered = clean_scene(file.into_iter().map(char::from).collect());
        std::fs::write(path, lowered).unwrap();
    }
}

fn clean_scene(scene: String) -> String {
    lowercase(scene)
}

fn lowercase(scene: String) -> String {
    scene.to_ascii_lowercase()
}

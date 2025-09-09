use multimap::MultiMap;
use itertools::Itertools;
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
    split_punctuation(order_lines(lowercase(scene)))
}

fn lowercase(scene: String) -> String {
    scene.to_ascii_lowercase()
}

fn split_punctuation(scene: String) -> String {
    scene.replace(";", "; ").replace(",", ", ").replace("  ", " ")
}

fn order_lines(scene: String) -> String {
    let lines = scene
        .lines()
        .skip(1)
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                line.split_once(";")
            }
        })
        .collect::<MultiMap<_, _>>();

    let mut ordered_scene = String::new();

    for kind in ["point", "colour", "texture", "material", "object"] {
        if let Some(lines_of_kind) = lines.get_vec(kind) {
            ordered_scene += &lines_of_kind.iter().map(|line| format!("{kind};{line}")).join("\n");
            ordered_scene += "\n\n"
        }
    }
    ordered_scene
}

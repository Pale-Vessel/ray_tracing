use itertools::Itertools;
use multimap::MultiMap;
use std::fs::read_dir;

pub fn clean_scenes() {
    for path in read_dir("scenes").unwrap() {
        let path = path.unwrap().path();
        let file = std::fs::read(&path).unwrap();
        if file.is_empty() {
            continue
        }
        let cleaned = clean_scene(file.into_iter().map(char::from).collect());
        std::fs::write(path, cleaned).unwrap();
    }
}

fn clean_scene(scene: String) -> String {
    split_punctuation(order_lines(lowercase(scene)))
}

fn lowercase(scene: String) -> String {
    scene.to_ascii_lowercase()
}

const PUNCTUATION_MARKS: [&str; 3] = [";", ",", "//"];

fn split_punctuation(mut scene: String) -> String {
    for mark in PUNCTUATION_MARKS {
        let mark_space = &format!("{mark} ");
        scene = scene.replace(mark, mark_space);
    }
    scene.replace("  ", " ")
}

fn order_lines(scene: String) -> String {
    let lines = scene.lines().collect::<Vec<_>>();
    let (camera_info, other_lines) = lines.split_at(1);
    let lines = other_lines
        .iter()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                line.split_once(";")
            }
        })
        .collect::<MultiMap<_, _>>();

    let mut ordered_scene = format!("{}\n\n", camera_info[0]);

    for kind in ["point", "colour", "texture", "material", "object"] {
        if let Some(lines_of_kind) = lines.get_vec(kind) {
            ordered_scene += &lines_of_kind
                .iter()
                .map(|line| format!("{kind};{line}"))
                .join("\n");
            ordered_scene += "\n\n"
        }
    }
    ordered_scene
}

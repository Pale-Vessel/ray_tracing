use crate::file_utils::clean_scenes::{order_scenes::order_lines, syntax_cleaner::{lowercase, split_punctuation}};

pub(super) fn clean_scene(scene: String) -> String {
    split_punctuation(order_lines(lowercase(scene)))
}

mod syntax_cleaner {
    pub(super) fn lowercase(scene: String) -> String {
        scene.to_ascii_lowercase()
    }

    const PUNCTUATION_MARKS: [&str; 3] = [";", ",", "//"];

    pub(super) fn split_punctuation(mut scene: String) -> String {
        for mark in PUNCTUATION_MARKS {
            let mark_space = &format!("{mark} ");
            scene = scene.replace(mark, mark_space);
        }
        scene.replace("  ", " ")
    }
}

mod order_scenes {
    use itertools::Itertools;
    use multimap::MultiMap;

    pub(super) fn order_lines(scene: String) -> String {
        let lines = scene.lines().collect::<Vec<_>>();
        let (camera_info, other_lines) = lines.split_at(1);
        let (sky_colours, other_lines) = other_lines.split_at(1);
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

        let mut ordered_scene =
            format!("{}\n{}\n\n", camera_info[0], sky_colours[0]);

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
}

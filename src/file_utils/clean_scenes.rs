use crate::file_utils::clean_scenes::{
    order_scenes::order_lines,
    syntax_cleaner::{clean_whitespace, lowercase, split_punctuation},
};

pub(super) fn clean_scene(scene: String) -> String {
    clean_whitespace(split_punctuation(order_lines(lowercase(scene))))
}

mod syntax_cleaner {
    pub(super) fn lowercase(scene: String) -> String {
        scene.to_ascii_lowercase()
    }

    const PUNCTUATION_MARKS: [&str; 3] = [";", ",", "//"];

    pub(super) fn split_punctuation(scene: String) -> String {
        PUNCTUATION_MARKS.into_iter().fold(scene, |scene, mark| {
            let mark_space = &format!("{mark} ");
            scene.replace(mark, mark_space)
        })
    }

    pub(super) fn clean_whitespace(mut scene: String) -> String {
        scene = scene
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n");

        let mut changed = scene.replace("\n\n\n", "\n\n");
        while scene != changed {
            scene = changed.clone();
            changed = scene.replace("\n\n\n", "\n\n");
        }
        scene
    }
}

mod order_scenes {
    use std::fmt::Write;
    use std::collections::HashMap;

    const LINE_TYPES: [&str; 5] =
        ["point", "colour", "texture", "material", "object"];

    pub(super) fn order_lines(scene: String) -> String {
        let lines = scene.lines().collect::<Vec<_>>();
        let (camera_info, other_lines) = lines.split_at(1);
        let (sky_colours, other_lines) = other_lines.split_at(1);

        let lines = other_lines
            .iter()
            .map(|line| line.split_once(";").unwrap_or((line, "")))
            .collect::<Vec<_>>();

        let ordered_scene =
            format!("{}\n{}\n\n", camera_info[0], sky_colours[0]);

        ordered_scene + &sort_if_comment(lines)
    }

    fn sort_if_comment(to_sort: Vec<(&str, &str)>) -> String {
        let line_order: HashMap<_, _> = LINE_TYPES
            .into_iter()
            .enumerate()
            .map(|(index, name)| (name, index))
            .collect();
        let mut lines = to_sort
            .clone()
            .into_iter()
            .filter(|(line_type, _)| LINE_TYPES.contains(line_type))
            .collect::<Vec<_>>();
        lines.sort_by_cached_key(|&(line_type, _)| line_order.get(line_type));
        let mut sorted = lines.into_iter();
        let mut output = String::new();

        for (line_type, _) in to_sort.iter() {
            if LINE_TYPES.contains(line_type) {
                let (line_type, line_content) = sorted.next().unwrap();
                let _ = write!(output, "{line_type}; {line_content}");
            } else {
                output += line_type
            }
            output.push('\n');
        }
        output
    }
}

use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashMap<String, u32>,
    child_dirs: Vec<Box<Self>>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            files: HashMap::new(),
            name: name.to_string(),
            child_dirs: Vec::new(),
        }
    }

    fn find_child(&mut self, name: &str) -> Option<&mut Box<Self>> {
        for child in self.child_dirs.iter_mut() {
            if child.name == name {
                return Some(child);
            }

            let search_result = child.find_child(name);
            if search_result.is_some() {
                return search_result;
            }
        }

        None
    }

    fn add_child(&mut self, dir: Self) {
        self.child_dirs.push(Box::new(dir));
    }

    #[allow(dead_code)]
    fn get_string(&self, deepness: usize) -> String {
        let mut out = format!("- {} (dir)", self.name).to_string();
        let mut indents = "".to_string();
        for _ in 0..deepness {
            indents.push_str("  ");
        }
        out.insert_str(0, &indents);
        out.push('\n');

        for child in self.child_dirs.iter() {
            out.push_str(&child.get_string(deepness + 1));
        }

        for (file_name, &file_size) in self.files.iter() {
            let mut file_indents = indents.clone();
            file_indents.insert_str(0, "  ");
            out.push_str(&format!(
                "{}file \"{}\" = {}\n",
                file_indents, file_name, file_size
            ));
        }

        out
    }

    fn list_of_under(&self, under: u32) -> Vec<(String, u32)> {
        let mut out = Vec::new();
        let own_size = self.size();
        if own_size <= under {
            out.push((self.name.clone(), own_size));
        }

        for child in self.child_dirs.iter() {
            out.append(&mut child.list_of_under(under));
        }
        
        out
    }
    /* fn sum_of_under(&self, under: u32) -> u32 {
        let self_size = self.size();
        let mut out = 0;
        if self_size <= under {
            out += self_size;
        }

        for child in self.child_dirs.iter() {
            out += child.sum_of_under(under);
        }

        out
    } */

    fn size(&self) -> u32 {
        let mut out = 0u32;

        for &file_size in self.files.values() {
            out += file_size
        }

        for child in self.child_dirs.iter() {
            out += child.size();
        }

        out
    }
}

pub fn day_7(input: &str) {
    let mut current_name = "/";
    let mut old_name = current_name;
    let mut master = Directory::new("MASTER DO NOT ACCESS");
    master.add_child(Directory::new(current_name));

    for line in input.lines() {
        if line.contains('$') {
            let mut spl = line.split(' ');
            let _dollar = spl.next();
            if let Some("ls") = spl.next() {
                continue;
            }

            let next_name = spl.next().unwrap();

            if next_name == ".." {
                current_name = old_name;
            } else {
                old_name = current_name;
                current_name = next_name;
            }
        } else if line.contains("dir") {
            let new_dir_name = line.split_whitespace().nth(1).unwrap();

            if master.find_child(new_dir_name).is_none() {
                master
                    .find_child(current_name)
                    .unwrap()
                    .add_child(Directory::new(new_dir_name));
            }
        } else {
            let mut split = line.split_whitespace();
            let (Some(file_size_str), Some(file_name)) = (split.next(), split.next()) else {
                unreachable!();
            };

            let Ok(file_size) = file_size_str.parse::<u32>() else {
                unreachable!();
            };

            master
                .find_child(current_name)
                .unwrap()
                .files
                .insert(file_name.to_string(), file_size);
        }
    }

    let root = master.find_child("/").unwrap();
    // println!("{}", root.get_string(0));
    // println!("Day 7 (part 1) {}", root.sum_of_under(100_000));
    let mut part1 = 0;
    for pair in root.list_of_under(100_000) {
        part1 += pair.1;
    }
    println!("Day 7 (part 1) {}", part1);
}

use std::fmt::Debug;

#[derive(Debug)]
struct Directory {
    child_dirs: Vec<Box<Self>>,
    name: String,
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {
            child_dirs: Vec::new(),
            files: Vec::new(),
            name,
        }
    }

    fn size(&self) -> i32 {
        let mut out = 0;
        for f in self.files.iter() {
            out += f.size;
        }

        for dir in self.child_dirs.iter() {
            out += dir.size();
        }

        out
    }

    fn get_child_dir(&mut self, name: &str) -> Option<&mut Box<Self>> {
        for v in self.child_dirs.iter_mut() {
            if v.name == name {
                return Some(v);
            }
            let child = v.get_child_dir(name);
            if child.is_some() {
                return child;
            }
        }

        None
    }

    fn under(&self, limit: i32) -> i32 {
        let current_size = self.size();
        let mut out = if current_size <= limit { current_size } else { 0 };

        for dir in self.child_dirs.iter() {
            out += dir.under(limit);
        }

        out
    }
}

pub fn day_7(input: &str) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut master_dir = Directory::new("/".to_string());
    let mut marker = "/";
    let mut old_markers = Vec::<&str>::new();

    // skip over the "cd /"
    for i in 1..lines.len() {
        let v = &lines[i];
        let Some(dollar_index) = v.find('$') else {
            continue;
        };

        let mut spl = v[dollar_index+1..v.len()].split_whitespace();
        let command = spl.next().unwrap();
        if command == "cd" {
            let name = spl.next().unwrap();
            if name == ".." {
                marker = old_markers.pop().unwrap();
            } else {
                old_markers.push(marker);
                marker = name;
            }
        } else if command == "ls" {
            let mut last = 0;
            for j in i + 1..lines.len() {
                if lines[j].contains('$') {
                    last = j;
                    break;
                } else if j == lines.len() - 1 {
                    last = j + 1;
                    break;
                }
            }

            let lists = &lines[i+1..last];
            for line in lists {
                let mut spl = line.split_whitespace();
                let prefix = spl.next().unwrap();
                if prefix == "dir" {
                    let dir_name = spl.next().unwrap().to_string();
                    if marker != "/" {
                        master_dir
                            .get_child_dir(marker)
                            .unwrap()
                            .child_dirs
                            .push(Box::new(Directory::new(dir_name)));
                    } else {
                        master_dir
                            .child_dirs
                            .push(Box::new(Directory::new(dir_name)));
                    }
                } else if let Ok(size) = prefix.parse::<i32>() {
                    let f = File {
                        name: spl.next().unwrap().to_string(),
                        size,
                    };
                    if marker != "/" {
                        master_dir.get_child_dir(marker).unwrap().files.push(f);
                    } else {
                        master_dir.files.push(f);
                    }
                }
            }
        }
    }

    println!("Day 7 (part 1) {}", master_dir.under(100_000));
}

use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

#[derive(Debug)]
pub struct MyFolder {
    name: String,
    size: i32,
    files: Vec<MyFile>,
    folders: Vec<Rc<RefCell<MyFolder>>>,
}

impl MyFolder {
    fn new(name: String) -> Self {
        MyFolder {
            name,
            size: 0,
            files: vec![],
            folders: vec![],
        }
    }

    fn folders_deep(&self) -> Vec<Rc<RefCell<MyFolder>>> {
        let child_iters = self
            .folders
            .iter()
            .flat_map(|folder| folder.borrow().folders_deep());

        self.folders.iter().cloned().chain(child_iters).collect()
    }
}

#[derive(Debug)]
struct MyFile {
    name: String,
    size: i32,
}

#[derive(Debug)]
struct MyFileSystem {
    root: Rc<RefCell<MyFolder>>,
    path: Vec<Rc<RefCell<MyFolder>>>,
}

impl MyFileSystem {
    fn new() -> Self {
        let mut file_system = Self {
            root: Rc::new(RefCell::new(MyFolder::new("/".to_string()))),
            path: vec![],
        };

        file_system.path.push(file_system.root.clone());

        file_system
    }

    fn current_dir_mut(&mut self) -> RefMut<MyFolder> {
        self.path.last_mut().unwrap().borrow_mut()
    }

    fn add_directory(&mut self, dir_name: String) {
        self.current_dir_mut()
            .folders
            .push(Rc::new(RefCell::new(MyFolder::new(dir_name))));
    }

    fn add_file(&mut self, file_name: String, file_size: i32) {
        self.current_dir_mut().files.push(MyFile {
            name: file_name,
            size: file_size,
        });

        self.path
            .iter_mut()
            .for_each(|folder| folder.borrow_mut().size += file_size);
    }

    fn change_directory(&mut self, path_str: &str) {
        match path_str {
            ".." => {
                self.path.pop();
            }
            dir_name => {
                let next_dir = {
                    let current_dir = self.current_dir_mut();
                    current_dir
                        .folders
                        .iter()
                        .find(|folder| folder.borrow().name == dir_name)
                        .unwrap()
                        .clone()
                };

                self.path.push(next_dir);
            }
        };
    }

    fn exec(&mut self, cmd: &str) {
        let words: Vec<&str> = cmd.split(' ').collect();

        match words[..] {
            ["$", "ls"] => {
                // Do nothing
            }
            ["$", "cd", path_str] => {
                self.change_directory(path_str);
            }
            ["dir", dir_name] => {
                self.add_directory(dir_name.to_string());
            }
            [file_size, file_name] if file_size.parse::<i32>().is_ok() => {
                self.add_file(file_name.to_string(), file_size.parse::<i32>().unwrap());
            }
            _ => {
                panic!("Invalid command: {cmd:?}");
            }
        }
    }

    fn folders(&self) -> Vec<Rc<RefCell<MyFolder>>> {
        let mut folders = self.root.borrow().folders_deep();
        folders.push(self.root.clone());
        folders
    }

    fn total_size(&self) -> i32 {
        self.root.borrow().size
    }
}

const MAX_SIZE: i32 = 100000;
const TOTAL_SPACE: i32 = 70000000;
const SPACE_REQUIRED: i32 = 30000000;

fn main() {
    let file = File::open("../input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    lines.next(); // Ignore first line: "$ cd /"

    let mut file_system = MyFileSystem::new();

    lines.for_each(|l| file_system.exec(&l.unwrap()));

    let folders = file_system.folders();

    let part_1: i32 = folders
        .iter()
        .filter_map(|f| -> Option<i32> {
            let folder = f.borrow();

            if folder.size <= MAX_SIZE {
                Some(folder.size)
            } else {
                None
            }
        })
        .sum();

    let need_to_clear_size: i32 = file_system.total_size() - TOTAL_SPACE + SPACE_REQUIRED;

    let part_2: i32 = folders
        .iter()
        .filter_map(|f| -> Option<i32> {
            let folder = f.borrow();

            if folder.size >= need_to_clear_size {
                Some(folder.size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    println!("Part 1: {part_1:?}");
    println!("Part 2: {part_2:?}");
}

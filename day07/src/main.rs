use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct File {
    name: String,
    size: i64,
}

impl File {
    fn new(name: String, size: i64) -> Self {
        File { name, size }
    }
}

// BUG : Introduced reference cycle here..
// Fix this by using the weak pointers to parent .. (Done !!)
#[derive(Debug)]
pub struct Directory {
    name: String,
    parent: Option<Weak<RefCell<Directory>>>,
    folders: HashMap<String, Rc<RefCell<Directory>>>,
    files: HashMap<String, File>,
}

impl Directory {
    fn new(name: String, parent: Option<Weak<RefCell<Directory>>>) -> Self {
        Directory {
            name,
            parent,
            folders: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn get_total_size(&self) -> i64 {
        let mut total_size = 0;
        for f in self.files.values() {
            total_size += f.size;
        }
        for d in self.folders.values() {
            let dir = d.borrow();
            let size = dir.get_total_size();
            total_size += size;
        }
        total_size
    }

    fn get_folder(&self, name: String) -> Option<&Rc<RefCell<Directory>>> {
        self.folders.get(&name)
    }

    fn add_folder(&mut self, directory_ptr: Rc<RefCell<Directory>>) {
        let name = directory_ptr.borrow().name.clone();
        self.folders.insert(name, directory_ptr);
    }

    fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }
}

fn main() {
    //let data = include_str!("./example.txt");
    let data = include_str!("./input.txt");
    let root = Directory::new(String::from("/"), None);
    let root_rc = Rc::new(RefCell::new(root));
    let mut current_directory = Rc::clone(&root_rc);
    let mut all_dirs = vec![];
    all_dirs.push(Rc::clone(&root_rc));
    for line in data.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let changed_dir;
        if parts[0] == "$" {
            // Command
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    let _cwd = current_directory.borrow();
                    match _cwd.parent.as_ref() {
                        Some(dir) => {
                            // SAFETY : we require trust that parent directory is never deleted
                            // And we have. Since this question does not have any rm operations
                            changed_dir = dir.upgrade().unwrap();
                        }
                        None => {
                            panic!("Invalid_move");
                        }
                    }
                } else if parts[2] == "/" {
                    changed_dir = Rc::clone(&root_rc);
                } else {
                    match current_directory
                        .borrow()
                        .get_folder(String::from(parts[2]))
                    {
                        Some(dir) => {
                            changed_dir = Rc::clone(dir);
                        }
                        None => {
                            panic!("Directory not found");
                        }
                    }
                }
                current_directory = changed_dir;
                //println!("{:?} {:?}", parts, current_directory.borrow().name);
            }
        } else {
            if parts[0] == "dir" {
                let new_dir =
                    Directory::new(String::from(parts[1]), Some(Rc::downgrade(&current_directory)));
                let new_dir_rc = Rc::new(RefCell::new(new_dir));
                current_directory
                    .borrow_mut()
                    .add_folder(Rc::clone(&new_dir_rc));
                all_dirs.push(Rc::clone(&new_dir_rc));
            } else {
                let new_file = File::new(
                    String::from(parts[1]),
                    String::from(parts[0]).parse::<i64>().unwrap(),
                );
                current_directory.borrow_mut().add_file(new_file);
            }
        }
    }

    println!("{:#?}", all_dirs);

    // part1
    let max_size = 100000;
    let mut ans1 = 0;
    for d in all_dirs.iter() {
        let dir = d.borrow();
        let total_size = dir.get_total_size();
        if total_size <= max_size {
            ans1 += total_size;
        }
    }

    // part2
    let disk_space = 70000000;
    let available = disk_space - root_rc.borrow().get_total_size();
    let need_to_free = 30000000 - available;

    let mut ans2 = i64::MAX;
    for d in all_dirs {
        let dir = d.borrow();
        let total_size = dir.get_total_size();
        if total_size >= need_to_free {
            ans2 = i64::min(ans2, total_size);
        }
    }

    println!("Part1 : {:?}", ans1);
    println!("Part2: {:?}", ans2);
}

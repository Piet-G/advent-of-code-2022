use std::collections::HashSet;
use std::rc::{Rc, Weak};
use regex::{Captures, Match, Regex};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;

struct File {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn get_size(&self) -> usize {
        self.files.iter().map(|file| file.size).sum::<usize>() + self.directories.iter().map(|dir| dir.borrow().get_size()).sum::<usize>()
    }

    fn get_sizes_under(&self, max_size: usize) -> usize {
        let own_size = if self.get_size() <= max_size { self.get_size() } else { 0 };

        return self.directories.iter().map(|dir| dir.borrow().get_sizes_under(max_size)).sum::<usize>() + own_size;
    }

    fn flatten(directory: &Rc<RefCell<Directory>>) -> Vec<Rc<RefCell<Directory>>> {
        let mut children: Vec<Rc<RefCell<Directory>>> = directory
            .borrow()
            .directories
            .iter()
            .map(|dir| Directory::flatten(dir))
            .flatten()
            .collect();

        children.push(Rc::clone(directory));

        return children;
    }

    fn get_smallest_valid_dir(directory: &Rc<RefCell<Directory>>, file_system_size: usize, min_size: usize) -> usize {
        let free_space= file_system_size - directory.borrow().get_size();

        Directory::flatten(directory)
            .iter()
            .map(|dir| dir.borrow().get_size())
            .filter(|size| *size >= min_size - free_space)
            .min()
            .unwrap()
    }

}

fn parse_file_tree(str: &str) -> Rc<RefCell<Directory>> {
    let root_dir = Rc::new(RefCell::new(Directory {
        name: "root".parse().unwrap(),
        files: vec![],
        directories: vec![],
        parent: None
    }));

    let mut current_dir: Rc<RefCell<Directory>> = Rc::clone(&root_dir);

    for directory_string in str.split("$ "){
        let cd_regex = Regex::new(r"cd (?P<name>.+)\n").unwrap();
        let ls_regex= Regex::new(r"(?s)ls\n(?P<dirlist>.*)").unwrap();

        match cd_regex.captures(directory_string) {
            Some(capture) => match capture.name("name").map(|name| name.as_str()) {
                None => {}
                Some("..") => {
                    current_dir = get_parent_dir(current_dir);
                }
                Some("/") => {
                    current_dir = Rc::clone(&root_dir);
                }
                Some(name) => {
                    current_dir = get_child_dir(current_dir, name);
                }
            }
            None => {
                match ls_regex.captures(directory_string) {
                    Some(capture) => {
                        parse_dir_content( Rc::clone(&current_dir), capture.name("dirlist").unwrap().as_str());
                    }
                    None => {}
                }
            }
        }
    }

    return Rc::clone(&root_dir);
}

fn get_parent_dir(current_dir: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
    Rc::clone(&current_dir.borrow().parent.as_ref().unwrap())
}

fn get_child_dir(current_dir: Rc<RefCell<Directory>>, name: &str) -> Rc<RefCell<Directory>> {
    let a: RefMut<Directory> = current_dir.borrow_mut();
    Rc::clone(a.directories.iter().find(|dir| dir.borrow_mut().name == name).unwrap())
}

fn parse_dir_content(directory: Rc<RefCell<Directory>>, str: &str) {
    let regex_dir = Regex::new(r"dir (?P<dir>.*)\n").unwrap();

    let directories: Vec<_> = regex_dir
        .captures_iter(str)
        .map(|mat| mat.name("dir").unwrap().as_str())
        .map(|name| Rc::new(RefCell::new(Directory{
            name: name.parse().unwrap(),
            parent: Some(Rc::clone(&directory)),
            files: vec![],
            directories: vec![],
        })))
        .collect();

    let regex_files = Regex::new(r"(?P<size>\d+) (?P<filename>.*)\n").unwrap();

    let files = regex_files
        .captures_iter(str)
        .map(|mat| File {
                name: mat.name("filename").unwrap().as_str().parse().unwrap(),
                size: mat.name("size").unwrap().as_str().parse::<usize>().unwrap()
        });

    let mut dir = directory.borrow_mut();
    dir.files.extend(files);
    dir.directories.extend(directories);
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(parse_file_tree(fs::read_to_string("src/day7/test_simple.txt").unwrap().as_mut_str()).borrow().get_sizes_under(100000), 95437);
    }

    #[test]
    fn large_test() {
        assert_eq!(parse_file_tree(fs::read_to_string("src/day7/test_large.txt").unwrap().as_mut_str()).borrow().get_sizes_under(100000), 1391690);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(Directory::get_smallest_valid_dir(&parse_file_tree(fs::read_to_string("src/day7/test_simple.txt").unwrap().as_mut_str()), 70000000, 30000000), 24933642);
    }

    #[test]
    fn large_test_2() {
        assert_eq!(Directory::get_smallest_valid_dir(&parse_file_tree(fs::read_to_string("src/day7/test_large.txt").unwrap().as_mut_str()), 70000000, 30000000), 5469168);
    }
}
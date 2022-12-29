use std::fmt::Display;
use std::fmt::Write;

use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

const CUTOFF: usize = 100000;
const TOTAL_SPACE: usize = 70000000;
const NEEDED: usize = 30000000;

#[derive(Debug)]
struct FileSystem {
    items: Vec<FSItem>,
}

#[derive(Debug)]
enum FSItem {
    Directory(Directory),
    File(File),
}

impl FSItem {
    fn is_directory(&self) -> bool {
        matches!(self, FSItem::Directory(_))
    }

    fn is_file(&self) -> bool {
        matches!(self, FSItem::File(_))
    }

    fn as_file(&self) -> &File {
        if let FSItem::File(f) = self {
            f
        } else {
            panic!("{:#?} is not a file", self)
        }
    }

    fn as_directory(&self) -> &Directory {
        if let FSItem::Directory(d) = self {
            d
        } else {
            panic!("{:#?} is not a directory", self)
        }
    }
    fn as_directory_mut(&mut self) -> &mut Directory {
        if let FSItem::Directory(d) = self {
            d
        } else {
            panic!("{:#?} is not a directory", self)
        }
    }

    fn parent(&self) -> Option<usize> {
        match self {
            FSItem::File(f) => Some(f.parent),
            FSItem::Directory(d) => d.parent,
        }
    }

    fn size(&self) -> usize {
        match self {
            FSItem::File(f) => f.size,
            FSItem::Directory(d) => d.size,
        }
    }

    fn display(
        &self,
        fs: &FileSystem,
        tab: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut prefix = String::new();
        for _ in 0..(tab * 2) {
            write!(&mut prefix, " ")?;
        }
        if self.is_file() {
            writeln!(
                f,
                "{}- {} (file, size={})",
                prefix,
                self.as_file().name,
                self.as_file().size
            )?;
        } else {
            writeln!(
                f,
                "{}- {} (dir, size={})",
                prefix,
                self.as_directory().name,
                self.size()
            )?;
            for &child in &self.as_directory().children {
                fs.items[child].display(fs, tab + 1, f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Directory {
    parent: Option<usize>,
    name: String,
    // index into the FileSystem
    children: Vec<usize>,
    size: usize,
}

#[derive(Debug)]
struct File {
    parent: usize,
    name: String,
    size: usize,
}

impl FileSystem {
    fn from_str(input: &str) -> Self {
        let mut items: Vec<FSItem> = Vec::new();
        let mut stack: Vec<usize> = Vec::new();
        let mut cwd = 0;

        for line in input.lines() {
            if line == "$ ls" {
            } else if line == "$ cd .." {
                stack.pop();
                cwd = stack[stack.len() - 1];
            } else if line.starts_with("$ cd ") {
                let name = line.split_at(5).1.to_string();
                if items.is_empty() {
                    items.push(FSItem::Directory(Directory {
                        name: "/".to_string(),
                        parent: None,
                        size: 0,
                        children: Vec::new(),
                    }));
                    stack.push(0);
                } else {
                    let mut found = false;
                    for &child in &items[cwd].as_directory().children {
                        if items[child].is_directory() {}
                        if items[child].is_directory() && items[child].as_directory().name == name {
                            found = true;
                            cwd = child;
                            stack.push(child);
                            break;
                        }
                    }
                    assert!(found);
                }
            } else {
                for (size, name) in line.split(' ').tuples() {
                    if let Ok(size) = size.parse::<usize>() {
                        let f = File {
                            name: name.to_string(),
                            parent: cwd,
                            size,
                        };
                        items.push(FSItem::File(f));
                    } else {
                        assert_eq!(size, "dir");
                        let d = Directory {
                            name: name.to_string(),
                            children: Vec::new(),
                            size: 0,
                            parent: Some(cwd),
                        };
                        items.push(FSItem::Directory(d));
                    }
                    let inserted = items.len() - 1;

                    items[cwd].as_directory_mut().children.push(inserted);
                }
            }
        }

        // propagate sizes
        for idx in (0..items.len()).rev() {
            if let Some(parent) = items[idx].parent() {
                items[parent].as_directory_mut().size += items[idx].size();
            }
        }

        FileSystem { items }
    }

    fn at_most_nodes(&self, cutoff: usize) -> usize {
        let mut stack = vec![0];
        let mut total = 0;

        while let Some(item) = stack.pop() {
            if self.items[item].size() <= cutoff {
                total += self.items[item].size();
            }
            for &child in &self.items[item].as_directory().children {
                if self.items[child].is_directory() {
                    stack.push(child);
                }
            }
        }

        total
    }

    fn find_at_least(&self, total: usize, needed: usize) -> usize {
        let used = self.items[0].size();
        let available = total - used;
        if available >= needed {
            return 0;
        }
        let to_find = needed - available;

        self.items
            .iter()
            .filter(|i| i.is_directory() && i.size() >= to_find)
            .map(|i| i.size())
            .k_smallest(1)
            .next()
            .unwrap()
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.items[0].display(self, 0, f)
    }
}

fn main() {
    let d = FileSystem::from_str(INPUT);
    println!("{}", d.at_most_nodes(CUTOFF));
    println!("{}", d.find_at_least(TOTAL_SPACE, NEEDED));
}

#[cfg(test)]
mod test {
    use crate::*;

    static TEST_INPUT: &str = r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_part_1() {
        let fs = FileSystem::from_str(TEST_INPUT);
        println!("{}", fs);
        assert_eq!(95437, fs.at_most_nodes(CUTOFF));
    }

    #[test]
    fn test_part_2() {
        let fs = FileSystem::from_str(TEST_INPUT);
        println!("{}", fs);
        assert_eq!(24933642, fs.find_at_least(TOTAL_SPACE, NEEDED));
    }
}

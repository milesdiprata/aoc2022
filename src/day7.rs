use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};
use std::rc::{Rc, Weak};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
enum Command {
    ChangeDir(String),
    List,
}

#[derive(Debug)]
enum FileData {
    Reg(usize),
    Dir(HashMap<String, File>),
}

#[derive(Debug)]
struct FileNode {
    name: String,
    data: FileData,
    parent: Option<Weak<RefCell<Self>>>,
}

#[derive(Clone, Debug)]
struct File(Rc<RefCell<FileNode>>);

impl FromStr for Command {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        let mut chars = str.chars();

        let prompt = chars
            .next()
            .ok_or_else(|| anyhow!("Missing command prompt!"))?;

        let command = ""
            .chars()
            .chain(chars.nth(1))
            .chain(chars.next())
            .collect::<String>();

        if prompt != '$' {
            return Err(anyhow!("Expected '$' as command prompt!"));
        }

        match command.as_str() {
            "cd" => Ok(Self::ChangeDir(chars.skip(1).collect())),
            "ls" => Ok(Self::List),
            _ => Err(anyhow!("Unknown command!")),
        }
    }
}

impl FromStr for File {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        if &str[0..3] == "dir" {
            let dir_name = str.chars().skip(4).collect();
            Ok(File::new_dir(dir_name))
        } else {
            let mut split = str.split_whitespace();
            let size = usize::from_str(split.next().ok_or_else(|| anyhow!("Missing file size!"))?)?;
            let file_name = split
                .next()
                .ok_or_else(|| anyhow!("Missing file name!"))?
                .to_owned();

            Ok(File::new_file(file_name, size))
        }
    }
}

impl File {
    fn from_node(node: FileNode) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    fn new_dir(dir: String) -> Self {
        Self::from_node(FileNode {
            name: dir,
            data: FileData::Dir(HashMap::new()),
            parent: None,
        })
    }

    fn new_file(file: String, size: usize) -> Self {
        Self::from_node(FileNode {
            name: file,
            data: FileData::Reg(size),
            parent: None,
        })
    }

    fn with_parent(self, file: Weak<RefCell<FileNode>>) -> Self {
        self.0.borrow_mut().parent = Some(file);
        self
    }

    fn to_size(&self) -> usize {
        match &self.0.borrow().data {
            FileData::Reg(size) => *size,
            FileData::Dir(files) => files.values().map(|file| file.to_size()).sum(),
        }
    }

    fn to_dirs(&self, is_root: bool) -> Result<Vec<Self>> {
        match &self.0.borrow().data {
            FileData::Dir(files) => Ok(if is_root { vec![] } else { vec![self.clone()] }
                .into_iter()
                .chain(
                    files
                        .values()
                        .flat_map(|file| file.to_dirs(false))
                        .flatten(),
                )
                .collect()),
            _ => Err(anyhow!("File not a directory!")),
        }
    }

    fn to_parent(&self) -> Option<Result<Self>> {
        self.0
            .borrow()
            .parent
            .clone()
            .map(|parent| parent.upgrade())
            .map(|parent| parent.ok_or_else(|| anyhow!("Parent reference dropped!")))
            .map(|parent| parent.map(Self))
    }

    fn find_flat(&self, file: &str) -> Result<Option<Self>> {
        match &self.0.borrow().data {
            FileData::Dir(files) => Ok(files.get(file).map(|file| file.0.clone()).map(Self)),
            _ => Err(anyhow!("File not a directory!")),
        }
    }

    fn insert(&mut self, other: Self) -> Result<()> {
        match &mut self.0.borrow_mut().data {
            FileData::Dir(contents) => {
                let name = other.0.borrow().name.as_str().to_owned();
                let parent = Rc::downgrade(&self.0);

                contents.insert(name, other.with_parent(parent));

                Ok(())
            }
            _ => Err(anyhow!("File not a directory!")),
        }
    }
}

fn read_files() -> Result<File> {
    let mut lines = io::stdin().lock().lines();
    let mut root = None;
    let mut file = None;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        if let Ok(cmd) = line.parse() {
            if let Command::ChangeDir(dir) = cmd {
                file = match dir.as_str() {
                    ".." => Some(
                        file.as_ref()
                            .and_then(|file: &File| file.to_parent())
                            .ok_or_else(|| anyhow!("File has no parent directory!"))??,
                    ),
                    _ => match file.as_ref() {
                        Some(file) => file.find_flat(dir.as_str())?,
                        None => Some(File::new_dir(dir)),
                    },
                }
            }
        } else {
            file.as_mut()
                .map(|file| file.insert(line.parse()?))
                .ok_or_else(|| anyhow!("No directory found!"))??;
        }

        if root.is_none() && file.is_some() {
            root = file.clone();
        } else if root.is_none() {
            break;
        }
    }

    root.ok_or_else(|| anyhow!("Missing root directory!"))
}

fn part_one(root: &File) -> Result<usize> {
    const MAX_DIR_SIZE: usize = 100000;

    root.to_dirs(false)
        .map(|dirs| dirs.into_iter())
        .map(|dirs| dirs.map(|dir| dir.to_size()))
        .map(|sizes| sizes.filter(|&size| size <= MAX_DIR_SIZE))
        .map(|sizes| sizes.sum())
}

fn part_two(root: &File) -> Result<Option<usize>> {
    const TOTAL_DISK_SIZE: usize = 70000000;
    const UPDATE_FREE_DISK_SIZE: usize = 30000000;

    let unused_disk_size = TOTAL_DISK_SIZE - root.to_size();

    let mut used_size = None;
    let mut used_sizes = root
        .to_dirs(false)
        .map(|dirs| dirs.into_iter())
        .map(|dirs| dirs.map(|dir| dir.to_size()))
        .map(|sizes| sizes.map(Reverse))
        .map(|sizes| sizes.collect::<BinaryHeap<_>>())?;

    while let Some(size) = used_sizes.pop() {
        if unused_disk_size + size.0 >= UPDATE_FREE_DISK_SIZE {
            used_size = Some(size.0);
            break;
        }
    }

    Ok(used_size)
}

fn main() -> Result<()> {
    let files = read_files()?;

    println!("Part one: {}", part_one(&files)?);
    println!(
        "Part two: {}",
        part_two(&files).map(|size| size.ok_or_else(|| anyhow!("No directory large enough!")))??
    );

    Ok(())
}

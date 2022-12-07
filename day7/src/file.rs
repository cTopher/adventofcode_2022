use std::str::{FromStr, Lines};

pub struct Directory {
    pub name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
}

pub struct File {
    pub name: String,
    size: usize,
}

impl Directory {
    const fn new(name: String) -> Self {
        Self {
            name,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    fn parse_lines(&mut self, lines: &mut Lines) {
        while let Some(line) = lines.next() {
            if let Some(cmd) = line.strip_prefix("$ ") {
                match cmd.strip_prefix("cd ") {
                    Some("..") => return,
                    Some(name) => {
                        let mut dir = Self::new(name.to_owned());
                        dir.parse_lines(lines);
                        self.directories.push(dir);
                    }
                    None => (),
                }
            } else if line.starts_with("dir") {
                // do nothing
            } else {
                self.files.push(line.parse().unwrap());
            }
        }
    }

    pub fn size(&self) -> usize {
        let dirs_size: usize = self.directories.iter().map(Self::size).sum();
        let files_size: usize = self.files.iter().map(|file| file.size).sum();
        dirs_size + files_size
    }

    pub fn redundant_size_sum(&self) -> usize {
        let sum = self.directories.iter().map(Self::redundant_size_sum).sum();
        if self.size() <= 100_000 {
            sum + self.size()
        } else {
            sum
        }
    }

    pub fn smallest_directory_greater_than(&self, minimum: usize) -> Option<usize> {
        self.directories
            .iter()
            .filter_map(|dir| dir.smallest_directory_greater_than(minimum))
            .min()
            .or_else(|| {
                if self.size() > minimum {
                    Some(self.size())
                } else {
                    None
                }
            })
    }
}

impl FromStr for File {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();
        let size = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap().to_owned();
        Ok(Self { name, size })
    }
}

impl FromStr for Directory {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut root = Self::new("/".to_owned());
        root.parse_lines(&mut input.lines());
        Ok(root)
    }
}

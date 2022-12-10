use super::{base::AocSolver, error::InputParseError};

struct File {
    size: usize,
}

struct Dir {
    name: String,
    size: usize,
    directories: Vec<Dir>,
    files: Vec<File>,
}

impl File {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl Dir {
    pub fn new(name: String) -> Self {
        Self {
            name,
            size: 0,
            directories: vec![],
            files: vec![],
        }
    }
}

struct WorkingDir<'a> {
    root: &'a mut Dir,
    path: Vec<usize>,
}

impl<'a> WorkingDir<'a> {
    pub fn new(root: &'a mut Dir) -> Self {
        Self { root, path: vec![] }
    }

    pub fn current(&self) -> &Dir {
        let mut pointer = &*self.root;
        for index in &self.path {
            pointer = &pointer.directories[*index]
        }
        pointer
    }

    fn current_mut(&mut self) -> &mut Dir {
        let mut pointer: &mut Dir = self.root;
        for index in &self.path {
            pointer = &mut pointer.directories[*index]
        }
        pointer
    }

    pub fn cd(&mut self, name: &str) {
        if name == "/" {
            self.path.clear();
            return;
        }
        if name == ".." {
            self.path.pop();
            return;
        }
        let current = self.current();
        for (i, dir) in current.directories.iter().enumerate() {
            if dir.name == name {
                self.path.push(i);
                return;
            }
        }
        // automagic dir creation
        self.path.push(current.directories.len());
        self.current_mut().directories.push(Dir::new(name.into()));
    }

    pub fn add_file(&mut self, file: File) {
        let mut pointer: &mut Dir = self.root;
        for index in &self.path {
            pointer.size += file.size;
            pointer = &mut pointer.directories[*index]
        }
        pointer.size += file.size;
        pointer.files.push(file);
    }

    pub fn add_dir(&mut self, new_dir: Dir) {
        let current = self.current_mut();
        let dirs = &mut current.directories;
        if dirs
            .iter()
            .find(|existing_dir| existing_dir.name == new_dir.name)
            .is_none()
        {
            dirs.push(new_dir);
        }
    }
}

struct TotalSizeOfDeletionCandidatesVisitor {
    total: usize,
}

impl TotalSizeOfDeletionCandidatesVisitor {
    pub fn visit(root: &Dir) -> usize {
        let mut visitor = Self { total: 0 };
        visitor.visit_dir(root);
        visitor.total
    }

    fn visit_dir(&mut self, dir: &Dir) {
        if dir.size <= 100_000 {
            self.total += dir.size;
        }
        for subdir in &dir.directories {
            self.visit_dir(subdir);
        }
    }
}

struct FindDirectoryToDeleteVisitor {
    deletion_size: usize,
    to_free: usize,
}

impl FindDirectoryToDeleteVisitor {
    pub fn visit(root: &Dir) -> usize {
        let available = 70_000_000 - root.size;
        let to_free = 30_000_000 - available;
        let mut visitor = Self {
            deletion_size: root.size,
            to_free,
        };
        visitor.visit_dir(root);
        visitor.deletion_size
    }

    fn visit_dir(&mut self, dir: &Dir) {
        if dir.size >= self.to_free && dir.size < self.deletion_size {
            self.deletion_size = dir.size;
        }
        for subdir in &dir.directories {
            self.visit_dir(subdir);
        }
    }
}

pub struct Solver {
    root: Dir,
}

impl AocSolver<usize> for Solver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut root = Dir::new("".into());
        let mut working_dir = WorkingDir::new(&mut root);
        for line in input {
            let line = line.trim();
            if line == "" {
                continue;
            } else if line.starts_with("$ cd ") {
                working_dir.cd(&line[5..].trim());
            } else if line.starts_with("$ ls") {
                // no op
            } else if line.starts_with("dir ") {
                working_dir.add_dir(Dir::new(line[4..].trim().into()));
            } else {
                let split_idx = line
                    .find(' ')
                    .ok_or_else(|| InputParseError::new("invalid file listing".into()))?;
                working_dir.add_file(File::new(line[..split_idx].parse()?))
            }
        }
        Ok(Self { root })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        Ok(TotalSizeOfDeletionCandidatesVisitor::visit(&self.root))
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        Ok(Some(FindDirectoryToDeleteVisitor::visit(&self.root)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = "\
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
        test_example_input::<Solver, _>(input, 95437, Some(24933642));
    }
}

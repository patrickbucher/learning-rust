use std::collections::VecDeque;
use std::fs::{self, read_dir, DirEntry};
use std::path::PathBuf;

pub fn list_files_breadth_first(path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut result: Vec<PathBuf> = Vec::new();
    let mut worklist: VecDeque<PathBuf> = VecDeque::from([path.clone()]);
    while !worklist.is_empty() {
        match worklist.pop_back() {
            Some(next) => {
                if next.is_dir() {
                    let entries = read_dir(next.clone())?;
                    for entry in entries {
                        let entry = entry?;
                        worklist.push_front(entry.path());
                    }
                } else {
                    result.push(next.clone());
                }
            }
            None => return Ok(result),
        }
    }
    Ok(result)
}

pub fn list_files_depth_first(path: &PathBuf) -> Vec<PathBuf> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::{create_dir, remove_dir_all, write};
    use std::path::PathBuf;

    #[test]
    fn breadth_first() {
        let nodes = ["foo", "bar", "qux"];
        let path = scaffold(&nodes);
        let expected: Vec<&str> = vec!["foo/foo.txt", "foo/bar/bar.txt", "foo/bar/qux/qux.txt"];
        let actual = list_files_breadth_first(&path).unwrap();
        for (i, path) in actual.iter().enumerate() {
            let path = path.clone().into_os_string().into_string().unwrap();
            assert!(path.contains(&expected[i]));
        }
        cleanup(&path);
    }

    fn scaffold(nodes: &[&str]) -> PathBuf {
        let mut dir = temp_dir();
        let mut root: Option<PathBuf> = None;
        for node in nodes {
            dir.push(node);
            if let None = root {
                root = Some(dir.clone());
            }
            let mut file = dir.clone();
            file.push(format!("{node}.txt"));
            create_dir(dir.clone());
            write(file, node);
        }
        root.unwrap()
    }

    fn cleanup(dir: &PathBuf) {
        remove_dir_all(dir);
    }
}

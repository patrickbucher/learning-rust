use std::collections::VecDeque;
use std::fs::read_dir;
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn list_files_breadth_first(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut result: Vec<PathBuf> = Vec::new();
    let mut worklist: VecDeque<PathBuf> = VecDeque::from([path.to_path_buf().clone()]);
    while !worklist.is_empty() {
        match worklist.pop_front() {
            Some(next) => {
                if next.is_dir() {
                    let entries = read_dir(next.clone())?;
                    for entry in entries {
                        let entry = entry?;
                        worklist.push_back(entry.path());
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

pub fn list_files_depth_first(path: &Path) -> Result<Vec<PathBuf>, Error> {
    if path.is_dir() {
        let entries = read_dir(path)?;
        let mut result: Vec<PathBuf> = Vec::new();
        for entry in entries {
            let entry = entry?;
            let sub_entries = list_files_depth_first(&entry.path())?;
            result.extend(sub_entries);
        }
        Ok(result)
    } else {
        Ok(vec![path.to_path_buf().clone()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::{create_dir, remove_dir_all, write};
    use std::io::ErrorKind;
    use std::path::PathBuf;

    #[test]
    #[ignore] // NOTE: only run with --test-threads 1
    fn new_breadth_first() -> Result<(), Error> {
        let path = create_scaffold(get_entry_hiera())?;
        let expected = vec![
            "abc/1.txt",
            "abc/2.txt",
            "abc/3.txt",
            "abc/def/4.txt",
            "abc/def/5.txt",
            "abc/def/6.txt",
            "abc/def/ghi/7.txt",
            "abc/def/ghi/8.txt",
            "abc/def/ghi/9.txt",
        ];
        let actual = list_files_breadth_first(&path)?;
        let mut actual: Vec<String> = actual
            .into_iter()
            .map(|p| p.into_os_string().into_string().unwrap())
            .collect();
        actual.sort();
        for (i, path) in actual.iter().enumerate() {
            assert!(path.contains(expected[i]));
        }
        cleanup(&path)?;
        Ok(())
    }

    #[test]
    #[ignore] // NOTE: only run with --test-threads 1
    fn new_depth_first() -> Result<(), Error> {
        let path = create_scaffold(get_entry_hiera())?;
        let expected = vec![
            "abc/1.txt",
            "abc/2.txt",
            "abc/3.txt",
            "abc/def/4.txt",
            "abc/def/5.txt",
            "abc/def/6.txt",
            "abc/def/ghi/7.txt",
            "abc/def/ghi/8.txt",
            "abc/def/ghi/9.txt",
        ];
        let actual = list_files_depth_first(&path)?;
        let mut actual: Vec<String> = actual
            .into_iter()
            .map(|p| p.into_os_string().into_string().unwrap())
            .collect();
        actual.sort();
        for (i, path) in actual.iter().enumerate() {
            assert!(path.contains(expected[i]));
        }
        cleanup(&path)?;
        Ok(())
    }

    fn create_scaffold(hiera: Entry) -> Result<PathBuf, Error> {
        let workdir = temp_dir();
        let mut root: Option<PathBuf> = None;
        let mut worklist: VecDeque<(PathBuf, Entry)> = VecDeque::from([(workdir, hiera)]);
        while !worklist.is_empty() {
            match worklist.pop_front() {
                Some((workdir, entry)) => match entry {
                    Entry::Dir { name, content } => {
                        let mut dir = workdir.clone();
                        dir.push(name);
                        create_dir(dir.clone())?;
                        if let None = root {
                            root = Some(dir.clone());
                        }
                        for child in content {
                            worklist.push_back((dir.clone(), child));
                        }
                    }
                    Entry::File { name, content } => {
                        let mut path = workdir.clone();
                        path.push(name);
                        write(path, content)?;
                    }
                },
                None => break,
            }
        }
        match root {
            Some(dir) => Ok(dir),
            None => Err(Error::new(ErrorKind::Other, "no scaffold was created")),
        }
    }

    enum Entry {
        Dir { name: String, content: Vec<Entry> },
        File { name: String, content: String },
    }

    fn get_entry_hiera() -> Entry {
        Entry::Dir {
            name: "abc".into(),
            content: vec![
                Entry::File {
                    name: "1.txt".into(),
                    content: "one".into(),
                },
                Entry::File {
                    name: "2.txt".into(),
                    content: "two".into(),
                },
                Entry::File {
                    name: "3.txt".into(),
                    content: "three".into(),
                },
                Entry::Dir {
                    name: "def".into(),
                    content: vec![
                        Entry::File {
                            name: "4.txt".into(),
                            content: "four".into(),
                        },
                        Entry::File {
                            name: "5.txt".into(),
                            content: "five".into(),
                        },
                        Entry::File {
                            name: "6.txt".into(),
                            content: "six".into(),
                        },
                        Entry::Dir {
                            name: "ghi".into(),
                            content: vec![
                                Entry::File {
                                    name: "7.txt".into(),
                                    content: "four".into(),
                                },
                                Entry::File {
                                    name: "8.txt".into(),
                                    content: "five".into(),
                                },
                                Entry::File {
                                    name: "9.txt".into(),
                                    content: "six".into(),
                                },
                            ],
                        },
                    ],
                },
            ],
        }
    }

    fn cleanup(dir: &PathBuf) -> Result<(), Error> {
        remove_dir_all(dir)
    }
}

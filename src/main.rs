use clap::Parser;
use std::{fs, path::PathBuf, process::exit};

#[derive(Parser, Debug)]
#[command(name = "not-rm")]
#[command(author = "yaxley peaks <epiclycoolgaemer@gmail.com>")]
#[command(version = "1.1")]
#[command(about = "Does not remove files with extensions in folder")]
struct CLI {
    /// Path to folder
    input: String,
    /// File extensions to NOT delete.
    /// Do not include the '.' (e.g. "txt" and not ".txt")
    exts: Vec<String>,
}

fn get_all_files(path: &str) -> Vec<PathBuf> {
    let mut res = vec![];
    let paths = fs::read_dir(path).unwrap();
    for _path in paths {
        if _path.as_ref().unwrap().path().is_file() {
            res.push(_path.unwrap().path());
        }
    }
    res
}
fn filter_not_paths(paths: Vec<PathBuf>, pat: &str) -> Vec<PathBuf> {
    paths
        .into_iter()
        .filter(|x| x.extension().unwrap_or_default().to_owned() != pat)
        .collect()
}
fn main() {
    let info = CLI::parse();

    if info.exts.len() == 0 {
        eprintln!("No file extensions were provided. Doing nothing!"); // add color here maybe
        exit(1);
    }

    for path in info.exts {
        filter_not_paths(get_all_files(&info.input), &path)
            .into_iter()
            .for_each(|file| fs::remove_file(file).expect("Failed to get access"));
    }
}

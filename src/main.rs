use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct CLI {
    input: String,
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
        .filter(|x| x.extension().unwrap().to_owned() == pat)
        .collect()
}
fn main() {
    let info = CLI::parse();
    for path in info.exts {
        filter_not_paths(get_all_files(&info.input), &path)
            .into_iter()
            .for_each(|file| fs::remove_file(file).expect("Failed to get access"));
    }
}

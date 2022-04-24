use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct CLI {
    input: String,
    ext: String,
}

fn get_all_files(path: &str) -> Vec<String> {
    let mut res = vec![];
    let paths = fs::read_dir(path).unwrap();
    for _path in paths {
        res.push(String::from(_path.unwrap().path().to_str().unwrap()));
    }
    res
}
fn filter_not_paths(paths: Vec<String>, pat: &str) -> Vec<String> {
    paths
        .into_iter()
        .filter(|path| !path.contains(pat))
        .collect()
}
fn main() {
    let path = CLI::parse();
    filter_not_paths(get_all_files(&path.input), &path.ext)
        .into_iter()
        .for_each(|file| fs::remove_file(file).unwrap());
}

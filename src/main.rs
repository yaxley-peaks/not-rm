use std::fs;

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
    filter_not_paths(get_all_files("./test"), ".txt")
        .into_iter()
        .for_each(|file| fs::remove_file(file).unwrap());
}

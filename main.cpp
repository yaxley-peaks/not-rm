#include <iostream>
#include <fstream>
#include <filesystem>
#include <string>
#include <vector>

namespace fs = std::filesystem;

typedef std::vector<std::string> str_vec;

str_vec get_all_files(const std::string &path) {
    str_vec res = str_vec();
    for(const fs::directory_entry& entry: fs::directory_iterator(path)){
        res.push_back(entry.path().string());
    }
    return res;
}

str_vec filter_not_paths(const str_vec& paths, const std::string& pat){
    str_vec res = str_vec();
    for (const std::string &item: paths)
        if (item.find(pat) != std::string::npos) res.push_back(item);
    return res;
}

struct CLI {
    std::string input;
    std::string pat;
};

std::unique_ptr<CLI> handle_args(char** args, int n){
    CLI cli = {};
    switch (n) {
        case 1:
        case 2:
            std::cerr << "error: The following required arguments were not provided:\n"
                         "    <INPUT>\n"
                         "    <EXT>\n"
                         "\n"
                         "USAGE:\n"
                         "    not-rm.exe <INPUT> <EXT>\n"
                         "    not-rm.exe <INPUT> <EXT>\n"
                         "\n"
                         "For more information try --help"
                     <<   std::endl;
            exit(-1);
        case 3:
            cli.input = std::string(args[1]);
            cli.pat = std::string(args[2]);
        default:
            break;
    }
    return std::make_unique<CLI>(cli);
}

int main(int argc, char** argv) {
    CLI args = *handle_args(argv, argc);
    std::cout << args.input << " -> " << args.pat << '\n';
    auto notPaths = filter_not_paths(get_all_files(args.input), args.pat);

    for (const std::string& notPath: notPaths) {
        std::cout << notPath << '\n';
        fs::path path_rm = notPath;
        if(fs::is_directory(path_rm)) continue;
        if(!fs::remove(path_rm)) std::cout << "Could not remove: " << path_rm << '\n';
    }
    return 0;
}

use regex::Regex;

#[derive(std::fmt::Debug)]
struct DirNode {
    dir: std::fs::DirEntry,
    children: Vec<DirNode>,
}

fn main() {
    parse_command();
}

fn parse_command() {
    let mut subcmd_args: Vec<String> = std::env::args().collect();
    subcmd_args = subcmd_args.split_off(2);
    match std::env::args().nth(1) {
        Some(inner) => match inner.as_str() {
            "echo" => echo(subcmd_args),
            "cat" => cat(subcmd_args),
            "ls" => ls(subcmd_args),
            "tree" => tree(subcmd_args),
            "grep" => grep(subcmd_args),
            _ => panic!("Unknown subcommand"),
        },
        None => panic!("No subcommand found."),
    }
}

fn echo(subcmd_args: Vec<String>) {
    println!("{}", subcmd_args.join(" "));
}

fn cat(subcmd_args: Vec<String>) {
    let mut files_contents: Vec<String> = Vec::new();
    for file_path in subcmd_args {
        files_contents.push(std::fs::read_to_string(file_path).unwrap());
    }
    println!("{}", files_contents.join(""));
}

fn ls(subcmd_args: Vec<String>) {
    let arg_count: usize = subcmd_args.len();
    if arg_count != 1 {
        panic!("ls only accepts 1 argument. {} provided.", arg_count)
    }

    let path_pattern: String = subcmd_args.into_iter().nth(0).unwrap();
    let mut file_list_info: Vec<String> = Vec::new();
    for f in std::fs::read_dir(path_pattern).unwrap() {
        match f {
            Ok(dir_entry) => {
                file_list_info.push(dir_entry.file_name().into_string().unwrap());
            }
            Err(_) => (),
        }
    }
    println!("{}", file_list_info.join(""));
}

fn tree(subcmd_args: Vec<String>) {
    let arg_count = subcmd_args.len();
    if arg_count > 1 {
        panic!("ls only accepts 1 argument. {} provided.", arg_count)
    }
    let path_pattern: String = subcmd_args.into_iter().nth(0).unwrap();
    let formatted_tree: Vec<String> = format_tree(iter_dir_recursive(path_pattern, -1), 1);
    println!("{}", formatted_tree.join(""));
}

fn format_tree(dir_nodes: Vec<DirNode>, indent_level: usize) -> Vec<String> {
    let mut tree_structure: Vec<String> = Vec::new();
    for node in dir_nodes {
        let filename: String = node.dir.file_name().into_string().unwrap();
        let indentation: String = format!("{}└──", "   ".repeat(indent_level - 1));
        tree_structure.push(format!("{}{}\n", indentation, filename));
        if node.children.len() > 0 {
            let mut r: Vec<String> = format_tree(node.children, indent_level + 1);
            tree_structure.append(&mut r);
        }
    }
    return tree_structure;
}

fn iter_dir_recursive(path_pattern: String, max_level: i32) -> Vec<DirNode> {
    let mut file_list_info: Vec<DirNode> = Vec::new();
    for f in std::fs::read_dir(&path_pattern).unwrap() {
        match f {
            Ok(dir_entry) => {
                let metadata: std::fs::Metadata = dir_entry.metadata().unwrap();
                let dir_entry_path: String = dir_entry.file_name().into_string().unwrap();
                let mut curr_dir_node: DirNode = DirNode {
                    dir: dir_entry,
                    children: Vec::new(),
                };
                if max_level == 0 {
                    return file_list_info;
                }
                if metadata.is_dir() {
                    let complete_path: String = format!("{}/{}", &path_pattern, &dir_entry_path);
                    curr_dir_node.children = iter_dir_recursive(complete_path, max_level - 1);
                }
                file_list_info.push(curr_dir_node);
            }
            Err(_) => (),
        }
    }
    return file_list_info;
}

fn grep(mut subcmd_args: Vec<String>) {
    if subcmd_args.len() != 2 {
        panic!(
            "grep expects 2 args. {} given.\nUsage: grep <pattern> <filepath>",
            subcmd_args.len()
        );
    }
    let file_path: String = subcmd_args.pop().unwrap();
    let pattern: String = subcmd_args.pop().unwrap();
    let file_contents: String = std::fs::read_to_string(&file_path).unwrap();
    let re: Regex = Regex::new(pattern.as_str()).unwrap();

    println!("{:#?}", re.captures_iter(&file_contents));

    // for (_, [path, lineno, line]) in re.captures_iter(&file_contents).map(|c| c.extract()) {
    //     println!("path: {}", path);
    //     println!("lineno: {}", lineno);
    //     println!("line: {}", line);
    // }

    // println!("Pattern: '{}' | File: '{}' | File contents: {}", pattern, file_path, file_contents);
}

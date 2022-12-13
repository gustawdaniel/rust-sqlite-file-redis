use std::env;

const FILE_BASE_PATCH: &str = "./collins-scrabble-2019";

#[test]
fn name_of_test_function_is_first_arg() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == String::from("name_of_test_function_is_first_arg") {
        assert_eq!(String::from("name_of_test_function_is_first_arg"), args[1]);
    }
}

#[test]
fn default_file_name() {
    let redis_host = &env::var("REDIS_HOST").unwrap_or("127.0.0.1".to_string())[..];

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == String::from("default_file_name") {
        assert_eq!(get_file_patch(None), FilePath {
            base: String::from("default_file_name"),
            text: String::from("default_file_name.txt"),
            tree: String::from("default_file_name-tree.txt"),
            tree_meta: String::from("default_file_name-tree.meta"),
            bin: String::from("default_file_name-bin"),
            sqlite: String::from("default_file_name.db"),
            redis_url: String::from("redis://".to_owned()+redis_host+"/"),
        });
    }
}

#[test]
fn position_out_of_scope() {
    assert_eq!(FILE_BASE_PATCH.to_owned(), "./collins-scrabble-2019");
    let redis_host = &env::var("REDIS_HOST").unwrap_or("127.0.0.1".to_string())[..];

    assert_eq!(get_file_patch(Some(1_000_000usize)), FilePath {
        base: FILE_BASE_PATCH.to_owned(),
        text: FILE_BASE_PATCH.to_owned() + ".txt",
        tree: FILE_BASE_PATCH.to_owned() + "-tree.txt",
        tree_meta: FILE_BASE_PATCH.to_owned() + "-tree.meta",
        bin: FILE_BASE_PATCH.to_owned() + "-bin",
        sqlite: FILE_BASE_PATCH.to_owned() + ".db",
        redis_url: String::from("redis://".to_owned()+redis_host+"/"),
    });
}

#[derive(Debug)]
pub struct FilePath {
    pub base: String,
    pub text: String,
    pub tree: String,
    pub tree_meta: String,
    pub bin: String,
    pub sqlite: String,
    pub redis_url: String,
}

impl PartialEq for FilePath {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.tree == other.tree && self.bin == other.bin && self.tree_meta == other.tree_meta
    }
}

pub fn get_file_patch(position: Option<usize>) -> FilePath {
    let position = position.unwrap_or(1usize);
    let args: Vec<String> = env::args().collect();
    let file_base = if args.len() > position { &args[position] } else { FILE_BASE_PATCH };
    let redis_host = &env::var("REDIS_HOST").unwrap_or("127.0.0.1".to_string())[..];

    FilePath {
        base: file_base.to_owned(),
        text: file_base.to_owned() + ".txt",
        tree: file_base.to_owned() + "-tree.txt",
        tree_meta: file_base.to_owned() + "-tree.meta",
        bin: file_base.to_owned() + "-bin",
        sqlite: file_base.to_owned() + ".db",
        redis_url: String::from("redis://".to_owned()+redis_host+"/"), // "redis://127.0.0.1/"
    }
}
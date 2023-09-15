use std::{
    env,
    fs::{self, DirEntry},
    path::Path,
};

const SCRIPT_PATH: &str = "../hinoki-parser/testdata/snapshots";

fn main() {
    println!("cargo:rerun-if-changed={}", SCRIPT_PATH);
    let mut test_list_string = String::from("static TEST_FILES: [&str;");

    println!("{}", env::current_dir().unwrap().to_str().unwrap());
    let dir = fs::read_dir(SCRIPT_PATH);

    let mut filenames: Vec<String> = vec![];

    match dir {
        Ok(files) => {
            let files = files.flatten().collect::<Vec<DirEntry>>();
            let num_files = files.len();
            test_list_string.push_str(&format!("{}] = [\n", num_files).to_owned());

            files.into_iter().for_each(|file| {
                let filename = file.file_name();
                let filename = filename.to_str().unwrap();
                let filename = filename.replace(".hi", "");

                println!("Found file: {}", filename);

                filenames.push(format!("\t\"{}\"", filename).to_owned());
            });
        }
        Err(e) => panic!("{}", e),
    };

    test_list_string.push_str(&filenames.join(",\n"));
    test_list_string.push_str("\n];\n");

    println!("{}", test_list_string);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("testfiles.rs");
    let _ = fs::write(dest_path, test_list_string);
}

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn write_words(name: &str, writer: &mut BufWriter<File>, length: usize) {
    let file = File::open(format!("words/{name}.txt")).unwrap();
    let reader = BufReader::new(file);

    writer
        .write(format!("const {}: [&str; {}] = [", name.to_uppercase(), length).as_bytes())
        .unwrap();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        writer.write(format!("\"{}\",", line).as_bytes()).unwrap();
    }
    writer.write("];".as_bytes()).unwrap();
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("words.rs");
    let out_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(out_file);

    write_words("animals", &mut writer, 593);
    write_words("adjectives", &mut writer, 28479);

    println!("cargo:rerun-if-changed=build.rs");
}

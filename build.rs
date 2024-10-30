use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

fn write_words(name: &str, writer: &mut BufWriter<File>, length: usize) {
    let file = File::open(format!("words/{name}.txt")).unwrap();
    let reader = BufReader::new(file);

    writer
        .write_all(format!("const {}: [&str; {}] = [", name.to_uppercase(), length).as_bytes())
        .unwrap();

    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        writer
            .write_all(format!("\"{}\",", line).as_bytes())
            .unwrap();
    }
    writer.write_all("];".as_bytes()).unwrap();
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

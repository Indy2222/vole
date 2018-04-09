use std::env;
use std::fs::OpenOptions;
use std::iter::Iterator;
use std::io::Error;
use std::io::Write;
use std::process;

fn main() {
    let mut args = env::args().skip(2);
    let value_a: String = next_arg(&mut args);
    let value_b: String = next_arg(&mut args);

    add_cmd(&value_a[..], &value_b[..]).unwrap();
}

fn next_arg<T: Iterator<Item = String>>(args: &mut T) -> String {
    match args.next() {
        Some(arg) => arg,
        None => {
            eprint!(
                "Invalid arguments, try: \
                 vole add <variant-a> <variant-b>"
            );
            process::exit(1);
        }
    }
}

fn add_cmd(value_a: &str, value_b: &str) -> Result<(), Error> {
    let mut file_path = match env::home_dir() {
        Some(path) => path,
        None => panic!("Can get your home dir."),
    };

    file_path.push(".vole");
    file_path.push("dictionary.txt");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    file.write(value_a.as_bytes())?;
    file.write(b"\x09")?;
    file.write(value_b.as_bytes())?;
    file.write(b"\x0A")?;
    file.flush()?;

    Result::Ok(())
}

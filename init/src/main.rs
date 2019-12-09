use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_input(filename: Option<&str>) -> impl Iterator<Item = String> {
    let filename: &str = filename.unwrap_or("input.txt");
    let file = File::open(filename);

    if let Ok(file) = file {
        let reader = io::BufReader::new(file);
        reader.lines().map(|l| l.unwrap())
    } else {
        panic!("Could not open file: {}", filename)
    }
}

fn main() {
    let filename = env::args().nth(1);
    let input = read_input(filename.as_ref().map(String::as_str));
    challenge(input);
}

fn challenge(input: impl Iterator<Item = String>) {
    // TODO: add solution here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec!["line1", "line2", "line3"];
        // assert_eq!(challenge(input.iter()), <some result>)
    }
}

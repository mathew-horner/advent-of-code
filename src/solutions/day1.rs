use std::io::Read;

pub fn solve(mut input: crate::Input) {
    let mut string = String::new();
    input.reader().read_to_string(&mut string).unwrap();
    println!("{string}");
}

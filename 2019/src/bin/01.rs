use std::env;
use std::fs;

fn read_file(filename: &str) {
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut sum = 0;
    for s in split {
        if s != "" {
            let fuel = ((s.parse::<f32>().unwrap() / 3.0).floor() as i32) - 2;
            sum += fuel;
        }
    }
    println!("Sum: {}", sum)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    read_file(filename);
}

use std::env;
use std::fs;

fn calc_fuel(fuel: f32) -> f32 {
    let needed = (fuel / 3.0).floor()  - 2.0;
    if needed <= 0.0 {
        return 0.0
    }
    return needed + calc_fuel(needed)
}

fn read_file(filename: &str) {
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut sum = 0;
    let mut sum2 = 0.0;
    for s in split {
        if s != "" {
            let fuel = ((s.parse::<f32>().unwrap() / 3.0).floor() as i32) - 2;
            let fuel2 = calc_fuel(s.parse::<f32>().unwrap());
            sum += fuel;
            sum2 += fuel2;
        }
    }
    println!("Sum: {}, Sum2: {}", sum, sum2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    read_file(filename);
}

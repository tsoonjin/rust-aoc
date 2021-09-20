fn read_file(path: &str) -> String {
    return path.to_string();
    format!("Hello, {}!", path)
}

fn main() {
    let display: String = read_file("Gabu");
    println!("{}", display);
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("{:?}", args);
    println!("query: {} \tfile: {}", query, file_name);
}

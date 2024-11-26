use std::fs::create_dir;

fn main() {
    //create 'output' directory
    match create_dir("output") {
        Ok(result) => println!("created output folder {:?}", result),
        Err(result) => println!("failed to create output folder {}", result),
    }
}

use std::process::Command;

fn main() {
    //create 'output' directory
    match Command::new("mkdir").arg("output").spawn() {
        Ok(result) => println!("created output folder {:?}", result),
        Err(result) => println!("failed to create output folder {}", result),
    }
}
use std::env;

fn main() {
    match env::var("HOME") {
        Ok(home) => {
            println!("Your home directory is: {}", home);
        },
        Err(_) => {
            println!("Couldn't read HOME environment variable.");
        }
    }
}

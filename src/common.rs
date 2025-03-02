use std::io::Write;
use std::time::Duration;

pub fn print_message(message: &str, duration: Duration) {
    for c in message.chars() {
        print!("{}", c);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(duration);
    }
    println!()
}

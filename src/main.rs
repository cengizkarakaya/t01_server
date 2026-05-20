mod heartbeat;

fn main() {

    println!("{}", "\n\x1b[38;2;0;135;255mt01 den merhaba!!!");

    if let Err(error) = heartbeat::heartbeat() {
        eprintln!("Heartbeat hatası: {error}");
    }

}

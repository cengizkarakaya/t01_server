mod heartbeat;

fn main() {

    println!("{}", "\n\x1b[38;2;0;135;255mTio'dan merhaba!!!\n\x1b[0m");

    if let Err(error) = heartbeat::heartbeat() {
        eprintln!("Heartbeat hatası: {error}");
    }

}

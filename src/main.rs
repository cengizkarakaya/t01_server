mod heartbeat;

fn main() -> std::io::Result<()> {
    println!("{}", "\n\x1b[38;2;0;135;255mt01 den merhaba!!!");
    heartbeat::heartbeat()?;
    Ok(())
}

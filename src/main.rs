mod heartbeat;

fn main() -> std::io::Result<()> {
    println!("{}", "t01 den merhaba!!!");
    heartbeat::heartbeat()?;
    Ok(())
}

use std::io::{self, Write};
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const LOCAL_BIND_ADDR: &str = "0.0.0.0:0";
const MAIN_PC_ADDR: &str = "192.168.1.3:5000";
const HEARTBEAT_INTERVAL_MS: u64 = 250;

pub fn heartbeat() -> std::io::Result<()> {
    let socket = UdpSocket::bind(LOCAL_BIND_ADDR)?;

    println!("\n\x1b[38;2;175;238;238mt01 UDP heartbeat sender başladı");
    println!("Ana-PC hedef adresi: {MAIN_PC_ADDR}");
    println!("Heartbeat aralığı: {HEARTBEAT_INTERVAL_MS} ms\n\x1b[0m");

    let mut seq: u32 = 0;

    loop {
        let timestamp_ms = current_time_ms();

        let msg = format!(
            "{{\"type\":\"heartbeat\",\"robot\":\"t01\",\"seq\":{},\"timestamp_ms\":{}}}",
            seq, timestamp_ms
        );

        socket.send_to(msg.as_bytes(), MAIN_PC_ADDR)?;

        print!("\r\x1B[2K\x1B[?7lheartbeat gönderildi: {msg}\x1B[?7h");
        io::stdout().flush()?;

        seq = seq.wrapping_add(1);
        thread::sleep(Duration::from_millis(HEARTBEAT_INTERVAL_MS));
    }
}

fn current_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

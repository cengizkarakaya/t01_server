use std::io::{self, Write};
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const LOCAL_BIND_ADDR: &str = "0.0.0.0:0";
const MAIN_PC_ADDR: &str = "192.168.1.3:5000";
const HEARTBEAT_INTERVAL_MS: u64 = 250;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const YELLOW: &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const WHITE: &str = "\x1b[97m";

const CLEAR_SCREEN: &str = "\x1b[2J";
const MOVE_CURSOR_HOME: &str = "\x1b[H";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";

pub fn heartbeat() -> io::Result<()> {
    let socket = UdpSocket::bind(LOCAL_BIND_ADDR)?;

    print!("{HIDE_CURSOR}");
    io::stdout().flush()?;

    let result = run_heartbeat_loop(&socket);

    print!("{SHOW_CURSOR}{RESET}");
    io::stdout().flush()?;

    result
}

fn run_heartbeat_loop(socket: &UdpSocket) -> io::Result<()> {
    let mut seq: u32 = 0;

    loop {
        let timestamp_ms = current_time_ms();
        let msg = build_heartbeat_message(seq, timestamp_ms);

        socket.send_to(msg.as_bytes(), MAIN_PC_ADDR)?;
        render_dashboard(seq, timestamp_ms, &msg)?;

        seq = seq.wrapping_add(1);
        thread::sleep(Duration::from_millis(HEARTBEAT_INTERVAL_MS));
    }
}

fn build_heartbeat_message(seq: u32, timestamp_ms: u128) -> String {
    format!(
        "{{\"type\":\"heartbeat\",\"robot\":\"t01\",\"seq\":{},\"timestamp_ms\":{}}}",
        seq, timestamp_ms
    )
}

fn render_dashboard(seq: u32, timestamp_ms: u128, msg: &str) -> io::Result<()> {
    print!("{CLEAR_SCREEN}{MOVE_CURSOR_HOME}");

    println!("{BOLD}{CYAN}╔══════════════════════════════════════════════════════════════╗{RESET}");
    println!("{BOLD}{CYAN}║{RESET} {BOLD}{GREEN}t01 UDP HEARTBEAT SENDER{RESET} {DIM}- Raspberry Pi Zero 2 W / tmux{RESET} {BOLD}{CYAN}║{RESET}");
    println!("{BOLD}{CYAN}╚══════════════════════════════════════════════════════════════╝{RESET}");
    println!();

    println!("{BOLD}{BLUE}┌─ DURUM ─────────────────────────────────────────────────────┐{RESET}");
    println!("{BOLD}{BLUE}│{RESET} {BOLD}{GREEN}● ÇALIŞIYOR{RESET}  {DIM}Heartbeat paketleri düzenli gönderiliyor{RESET}        {BOLD}{BLUE}│{RESET}");
    println!("{BOLD}{BLUE}└─────────────────────────────────────────────────────────────┘{RESET}");
    println!();

    println!("{BOLD}{MAGENTA}┌─ BAĞLANTI ──────────────────────────────────────────────────┐{RESET}");
    print_row("Robot", "t01");
    print_row("Yerel bind", LOCAL_BIND_ADDR);
    print_row("Ana-PC hedef", MAIN_PC_ADDR);
    print_row("Aralık", format_args!("{HEARTBEAT_INTERVAL_MS} ms"));
    println!("{BOLD}{MAGENTA}└─────────────────────────────────────────────────────────────┘{RESET}");
    println!();

    println!("{BOLD}{YELLOW}┌─ SON HEARTBEAT ──────────────────────────────────────────────┐{RESET}");
    print_row("Seq", seq);
    print_row("Timestamp_ms", timestamp_ms);
    println!("{BOLD}{YELLOW}│{RESET} {BOLD}{CYAN}{:<13}{RESET}: {WHITE}{}{RESET}", "JSON", msg);
    println!("{BOLD}{YELLOW}└─────────────────────────────────────────────────────────────┘{RESET}");
    println!();

    println!("{DIM}Çıkmak için Ctrl+C  •  Ekran her pakette yeniden çizilir, terminal dolmaz.{RESET}");

    io::stdout().flush()
}

fn print_row(label: &str, value: impl std::fmt::Display) {
    println!(
        "{BOLD}{BLUE}│{RESET} {BOLD}{CYAN}{label:<13}{RESET}: {WHITE}{value}{RESET}"
    );
}

fn current_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

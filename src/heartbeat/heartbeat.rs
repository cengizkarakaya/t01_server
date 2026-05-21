use std::io::{self, Result, Write, stdout};
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use terminal_colors::*;

//config:
const LOCAL_BIND_ADDR: &str = "0.0.0.0:0";
const MAIN_PC_ADDR: &str = "192.168.1.3:5000";
const HEARTBEAT_INTERVAL_MS: u64 = 250;

pub fn heartbeat() -> Result<()> {
    let socket = UdpSocket::bind(LOCAL_BIND_ADDR)?;

    write!(stdout(), "{CLEAR_SCREEN}{MOVE_CURSOR_HOME}{HIDE_CURSOR}")?;
    stdout().flush()?;

    let result = run_heartbeat_loop(&socket);

    write!(stdout(), "{SHOW_CURSOR}{RESET}")?;
    stdout().flush()?;

    result
}

fn run_heartbeat_loop(socket: &UdpSocket) -> Result<()> {
    let mut seq: u32 = 0;
    let mut heart_is_on = false;

    loop {
        heart_is_on = !heart_is_on;

        let heart_symbol = match heart_is_on {
            true => "\x1b[1;31m❤︎\x1b[0m",
            false => "\x1b[2;31m❤︎\x1b[0m",
        };

        let timestamp_ms = current_time_ms();
        let msg = build_heartbeat_message(seq, timestamp_ms);

        socket.send_to(msg.as_bytes(), MAIN_PC_ADDR)?;

        render_dashboard(seq, timestamp_ms, &msg, heart_symbol)?;

        seq = seq.wrapping_add(1);
        thread::sleep(Duration::from_millis(HEARTBEAT_INTERVAL_MS));
    }
}

fn build_heartbeat_message(seq: u32, timestamp_ms: u128) -> String {
    format!(
        "{{\"type\":\"heartbeat\",\"robot\":\"t01\",
                  \"seq\":{},\"timestamp_ms\":{}}}",
        seq, timestamp_ms
    )
}

fn render_dashboard(seq: u32, timestamp_ms: u128, msg: &str, heart_symbol: &str) -> Result<()> {
    write!(stdout(), "{MOVE_CURSOR_HOME}")?;

    writeln!(stdout(), "{}", "-".repeat(58))?;
    writeln!(
        stdout(),
        "{BOLD} \x1b[48;5;28m\x1b[38;5;15mt01 HEARTBEAT{RESET} {heart_symbol}{DIM}\n Raspberry Pi Zero 2 W / UDP{RESET}"
    )?;
    writeln!(stdout(), "{}\n", "-".repeat(58))?;

    writeln!(
        stdout(),
        "{BOLD}{C070_CHARTREUSE3}-- DURUM {}{RESET}",
        "-".repeat(49)
    )?;
    writeln!(
        stdout(),
        "{BOLD}{C004_NAVY_SYSTEM}{RESET} {BOLD}{C002_GREEN_SYSTEM}● ÇALIŞIYOR{RESET}  {DIM}Heartbeat paketleri düzenli gönderiliyor{RESET}       {BOLD}{C004_NAVY_SYSTEM}{RESET}"
    )?;
    writeln!(
        stdout(),
        "{BOLD}{C070_CHARTREUSE3}{}{RESET}\n",
        "-".repeat(58)
    )?;

    write!(
        stdout(),
        "{BOLD}{C011_YELLOW_SYSTEM}-- BAĞLANTI {}\n{RESET}",
        "-".repeat(46)
    )?;
    print_row("Robot", "t01")?;
    print_row("Yerel bind", LOCAL_BIND_ADDR)?;
    print_row("Ana-PC hedef", MAIN_PC_ADDR)?;
    print_row("Aralık", format_args!("{HEARTBEAT_INTERVAL_MS} ms"))?;
    write!(
        stdout(),
        "{BOLD}{C011_YELLOW_SYSTEM}{}\n\n{RESET}",
        "-".repeat(58)
    )?;

    write!(
        stdout(),
        "{BOLD}{C011_YELLOW_SYSTEM}-- SON HEARTBEAT {}\n{RESET}",
        "-".repeat(41)
    )?;
    print_row("Seq", seq)?;
    print_row("Timestamp_ms", timestamp_ms)?;
    write!(
        stdout(),
        "{BOLD}{C006_TEAL_SYSTEM}{:<13}{RESET}: {C015_WHITE_SYSTEM}{}{RESET}",
        "JSON",
        msg
    )?;
    write!(
        stdout(),
        "\n{BOLD}{C011_YELLOW_SYSTEM}{}\n{RESET}",
        "-".repeat(58)
    )?;

    write!(stdout(), "{DIM}Çıkmak için Ctrl+C{RESET}")?;

    io::stdout().flush()
}

fn print_row(label: &str, value: impl std::fmt::Display) -> Result<()> {
    writeln!(
        stdout(),
        "{BOLD}{C004_NAVY_SYSTEM}{RESET} {BOLD}{C006_TEAL_SYSTEM}{label:<13}{RESET}: {C015_WHITE_SYSTEM}{value}{RESET}"
    )?;
    Ok(())
}

fn current_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

// Bu program Raspberry Pi Zero 2 W üzerinde çalışır.
// Tio, ana-PC'ye UDP heartbeat paketleri gönderir.
//
// Program ekrana hiçbir şey yazmaz. Bu sürüm headless kullanım için uygundur:
// SSH, systemd service veya kutu içinde çalışan robot yazılımı gibi durumlarda
// terminal dashboard gereksiz I/O yükü oluşturmaz.
//
// Her heartbeat paketinde:
// - type
// - robot
// - seq
// - timestamp_ms
//
// alanları bulunur. Bu alanlara ekleme veya çıkarma yapılmamıştır.
// Yalnızca robot adı "Tio" olarak değiştirilmiştir.

use std::io::{Error, ErrorKind, Result};
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// =======================
// Config
// =======================

// Tio, herhangi bir sabit yerel port dinlemek zorunda değildir.
// "0.0.0.0", kullanılabilir tüm IPv4 ağ arayüzlerinden çıkış yapılabilmesini sağlar.
// ":0", yerel portu işletim sisteminin otomatik seçmesi demektir.
const LOCAL_BIND_ADDR: &str = "0.0.0.0:0";

// Heartbeat paketlerinin gönderileceği ana-PC adresi.
// Ana-PC tarafında bu IP ve portta UDP dinleyici çalışmalıdır.
const MAIN_PC_ADDR: &str = "192.168.1.10:5000";

// Robot adı. Gönderilen JSON içindeki "robot" alanında kullanılır.
const ROBOT_NAME: &str = "Tio";

// 250 ms = saniyede 4 heartbeat paketi.
// Bu, Pi Zero 2 W için hafif bir yük oluşturur ve bağlantı kopmasını hızlı fark etmek için uygundur.
// 100 ms çoğu durumda gereksiz sık olur.
// 1000 ms ise robot bağlantı takibi için biraz yavaş kalabilir.
const HEARTBEAT_INTERVAL_MS: u64 = 250;

// UDP socket'ini hazırlar ve heartbeat döngüsünü başlatır.
//
// `Result<()>`, socket oluşturma veya gönderim sırasında oluşabilecek I/O hatalarının
// çağırana aktarılmasını sağlar. Bu fonksiyon ekrana yazı yazmaz.
pub fn heartbeat() -> Result<()> {
    let socket = UdpSocket::bind(LOCAL_BIND_ADDR)?;

    run_heartbeat_loop(&socket)
}

// Verilen UDP socket üzerinden düzenli heartbeat paketleri gönderir.
//
// `&UdpSocket`, socket'in ownership'ini almaz; sadece borrow eder.
// Böylece socket bu fonksiyona taşınmadan kullanılabilir.
//
// Döngü sonsuzdur. Program çalıştığı sürece Tio, ana-PC'ye heartbeat göndermeye devam eder.
fn run_heartbeat_loop(socket: &UdpSocket) -> Result<()> {
    let mut seq: u32 = 0;

    loop {
        let timestamp_ms = current_time_ms();
        let msg = build_heartbeat_message(seq, timestamp_ms);

        send_heartbeat(socket, &msg)?;

        // `u32` en büyük değerine ulaştığında 0'a sarar.
        // Uzun süre çalışan sistemlerde taşma hatası oluşmasını engeller.
        seq = seq.wrapping_add(1);

        // CPU'yu boş yere meşgul etmemek ve heartbeat hızını sabit tutmak için
        // her gönderimden sonra belirtilen süre kadar beklenir.
        thread::sleep(Duration::from_millis(HEARTBEAT_INTERVAL_MS));
    }
}

// Heartbeat mesajını ana-PC'ye gönderir.
//
// UDP hızlı ve hafiftir, fakat teslim garantisi vermez.
// Bu yüzden ana-PC tarafı `seq` değerlerini takip ederek paket kaybını anlayabilir.
//
// `send_to`, gönderilen byte sayısını döndürür.
// UDP datagram normalde ya tamamen gönderilir ya da hata verir; yine de byte sayısı
// kontrol edilerek eksik gönderim durumunda hata üretilir.
fn send_heartbeat(socket: &UdpSocket, msg: &str) -> Result<()> {
    let sent_bytes = socket.send_to(msg.as_bytes(), MAIN_PC_ADDR)?;

    if sent_bytes != msg.len() {
        return Err(Error::new(
            ErrorKind::Other,
            "UDP heartbeat paketi eksik gönderildi",
        ));
    }

    Ok(())
}

// Sıra numarası ve timestamp değerini JSON benzeri bir String içinde birleştirir.
//
// Gönderilen alanlar:
// - type
// - robot
// - seq
// - timestamp_ms
//
// Bu yapıyı ana-PC tarafındaki alıcı program parse edebilir.
fn build_heartbeat_message(seq: u32, timestamp_ms: u128) -> String {
    format!(
        "{{\"type\":\"heartbeat\",\"robot\":\"{}\",\"seq\":{},\"timestamp_ms\":{}}}",
        ROBOT_NAME, seq, timestamp_ms
    )
}

// Şimdiki zamanı UNIX epoch'tan beri geçen milisaniye olarak hesaplar.
//
// UNIX epoch:
// 1970-01-01 00:00:00 UTC
//
// `SystemTime::now()` sistem saatini verir.
// `duration_since(UNIX_EPOCH)` epoch'tan bu ana kadar geçen süreyi hesaplar.
// Sistem saati garip biçimde epoch'tan önce görünürse panic yerine 0 döner.
fn current_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

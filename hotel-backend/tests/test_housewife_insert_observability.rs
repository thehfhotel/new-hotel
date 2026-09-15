//! Exercise the real mark-clean recipe, timeout helper, Tiberius row decoding,
//! and LegacyIds JSON together. A loopback TDS peer supplies deterministic
//! INSERT outcomes; no SQL Server or production database is contacted.
//!
//! This is a protocol fixture, not a SQL interpreter: SQL policy/parity stays
//! pinned by mark_clean's existing unit tests. Here a missing suffix, lost
//! result set, inverted boolean, or missing persisted field must fail.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use hotel_backend::db::PoisonAwareManager;
use hotel_backend::writeback::error::WritebackError;
use hotel_backend::writeback::recipes::mark_clean;

fn receive(stream: &mut TcpStream, expected_kind: u8) -> Vec<u8> {
    let mut body = Vec::new();
    loop {
        let mut header = [0; 8];
        stream.read_exact(&mut header).expect("TDS request header");
        assert_eq!(header[0], expected_kind);
        let length = u16::from_be_bytes([header[2], header[3]]) as usize;
        assert!(length >= 8);
        let start = body.len();
        body.resize(start + length - 8, 0);
        stream
            .read_exact(&mut body[start..])
            .expect("TDS request body");
        if header[1] & 1 != 0 {
            return body;
        }
    }
}

fn respond(stream: &mut TcpStream, body: &[u8]) {
    let length = u16::try_from(body.len() + 8).unwrap().to_be_bytes();
    stream
        .write_all(&[4, 1, length[0], length[1], 0, 0, 1, 0])
        .unwrap();
    stream.write_all(body).unwrap();
}

fn done(status: u16, count: u64) -> Vec<u8> {
    let mut token = vec![0xfd]; // DONE, current command 0, TDS 7.4 row count.
    token.extend(status.to_le_bytes());
    token.extend([0, 0]);
    token.extend(count.to_le_bytes());
    token
}

fn sql_batch(stream: &mut TcpStream) -> String {
    let body = receive(stream, 1);
    // SQLBatch ALL_HEADERS length includes the length field itself.
    let header_len = u32::from_le_bytes(body[..4].try_into().unwrap()) as usize;
    let utf16: Vec<u16> = body[header_len..]
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();
    String::from_utf16(&utf16).unwrap()
}

fn int_result(value: i32) -> Vec<u8> {
    let name = "housewife_rows_inserted";
    let mut tokens = vec![0x81, 1, 0]; // COLMETADATA, one column.
    tokens.extend([0; 6]); // user type (u32), flags (u16).
    tokens.push(0x38); // INT4, matching @@ROWCOUNT's SQL type.
    tokens.push(name.len() as u8);
    for unit in name.encode_utf16() {
        tokens.extend(unit.to_le_bytes());
    }
    tokens.push(0xd1); // ROW.
    tokens.extend(value.to_le_bytes());
    tokens.extend(done(0, 0));
    tokens
}

async fn run_recipe(count: Option<i32>) -> Result<serde_json::Value, WritebackError> {
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        receive(&mut stream, 0x12); // PRELOGIN.
                                    // ENCRYPTION option at offset 6: ENCRYPT_NOT_SUP, matching the plain
                                    // legacy connection. The fixture listens on loopback only.
        respond(&mut stream, &[1, 0, 6, 0, 1, 0xff, 2]);
        receive(&mut stream, 0x10); // LOGIN7.
        let mut login = vec![0xad, 10, 0, 1, 0x74, 0, 0, 4, 0, 0, 0, 0, 0];
        login.extend(done(0, 0));
        respond(&mut stream, &login);

        let prior = sql_batch(&mut stream);
        assert!(prior.starts_with("SELECT TOP 1 h.Cin_no,"));
        assert!(prior.contains("d.Cin_Room_No = 'TEST'"));
        respond(&mut stream, &done(0, 0)); // No prior occupant.

        assert_eq!(
            sql_batch(&mut stream),
            "update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=999"
        );
        respond(&mut stream, &done(0x10, 1));

        let insert = sql_batch(&mut stream);
        assert!(insert.starts_with("INSERT INTO HT_Housewife "));
        assert!(insert.contains("h_date > DATEADD(minute, -5, GETDATE())"));
        assert!(insert.ends_with("; SELECT @@ROWCOUNT AS housewife_rows_inserted"));
        let response = match count {
            Some(value) => {
                // INSERT's DONE_COUNT precedes the SELECT rowset, as on SQL
                // Server. This catches confusing the first DONE with a result.
                let mut tokens = done(0x11, u64::from(value == 1));
                tokens.extend(int_result(value));
                tokens
            }
            None => done(0, 0),
        };
        respond(&mut stream, &response);
    });

    let mut config = tiberius::Config::new();
    config.host("127.0.0.1");
    config.port(port);
    config.authentication(tiberius::AuthMethod::sql_server("SAMPLE", "SAMPLE"));
    config.encryption(tiberius::EncryptionLevel::NotSupported);
    let pool = bb8::Pool::builder()
        .max_size(1)
        .test_on_check_out(false)
        .connection_timeout(Duration::from_secs(5))
        .build(PoisonAwareManager::new(config))
        .await
        .unwrap();
    let mut conn = pool.get().await.unwrap();
    let result = mark_clean::execute(&mut conn, "TEST", 999, "SAMPLE").await;
    drop(conn);
    drop(pool);
    server.join().expect("scripted TDS peer assertions");
    result.map(|ids| serde_json::to_value(ids).unwrap())
}

#[tokio::test]
async fn inserted_and_suppressed_results_reach_persisted_json() {
    for count in [0, 1] {
        let ids = run_recipe(Some(count)).await.unwrap();
        assert_eq!(ids["extra"]["housewife_inserted"], count == 1);
        assert_eq!(ids["extra"]["room_id"], 999);
        assert!(ids["extra"].get("prior_cin_no").is_none());
    }
}

#[tokio::test]
async fn absent_or_unexpected_observation_cannot_be_persisted_as_suppression() {
    for count in [None, Some(2)] {
        assert!(matches!(
            run_recipe(count).await,
            Err(WritebackError::Recipe(_))
        ));
    }
}

use chrono::{DateTime, NaiveDate, Utc};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio}; // Add methods on commands

fn run_command(cmd: &str, input: &[u8]) -> Vec<u8> {
    let mut child = Command::new(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input).unwrap();
        // Close stdin to signal EOF to the child
        drop(stdin);
    }

    if let Some(stdout) = child.stdout.take() {
        let mut reader = BufReader::new(stdout);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer).unwrap();
        buffer
    } else {
        panic!("Failed to capture stdout");
    }
}

#[test]
fn test_datetime_to_sp() {
    let dt = DateTime::parse_from_rfc3339("2025-10-03T09:30:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let unix_ts = dt.timestamp() as u32; // Convert to Unix timestamp
    let input = unix_ts.to_le_bytes();
    let expected = 22u8.to_le_bytes();

    let buffer = run_command(env!("CARGO_BIN_EXE_datetime-to-sp"), &input);

    assert_eq!(buffer, expected);
}

#[test]
fn test_datetime_to_settlement_date() {
    let dt = DateTime::parse_from_rfc3339("2025-10-03T09:30:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let unix_ts = dt.timestamp() as u32; // Convert to Unix timestamp
    let input = unix_ts.to_le_bytes();
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let settlement_day_u16 =
        (NaiveDate::from_ymd_opt(2025, 10, 3).unwrap() - epoch).num_days() as u16;
    let expected = settlement_day_u16.to_le_bytes();

    let buffer = run_command(env!("CARGO_BIN_EXE_datetime-to-settlement-date"), &input);

    assert_eq!(buffer, expected);
}

#[test]
fn test_sp_to_datetime() {
    let sp = 22u8;
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let settlement_day_u16 =
        (NaiveDate::from_ymd_opt(2025, 10, 3).unwrap() - epoch).num_days() as u16;
    let input = [&settlement_day_u16.to_le_bytes()[..], &sp.to_le_bytes()[..]].concat();
    let dt = DateTime::parse_from_rfc3339("2025-10-03T09:30:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let expected = (dt.timestamp() as u32).to_le_bytes();

    let buffer = run_command(env!("CARGO_BIN_EXE_sp-to-datetime"), &input);

    assert_eq!(buffer, expected);
}

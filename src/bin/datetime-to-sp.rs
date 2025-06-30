use chrono::{TimeZone, Utc};
use clickhouse_udfs::gb_electricity::datetime_to_sp;
use clickhouse_udfs::row_binary_io::{
    parse_bytes_by_types, serialize_parsed_values, DataType, ParsedValue,
};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let types = [DataType::U32];
    for line in stdin.lock().lines() {
        match line {
            Ok(bytes) => {
                let bytes = bytes.as_bytes();
                match parse_bytes_by_types(bytes, &types) {
                    Ok(parsed) => {
                        if let ParsedValue::U32(ts) = parsed[0] {
                            let dt = Utc.timestamp_opt(ts as i64, 0).unwrap();
                            let (_settlement_day, settlement_period) = datetime_to_sp(dt);
                            let output =
                                serialize_parsed_values(&[ParsedValue::U8(settlement_period)]);
                            stdout.write_all(&output).unwrap();
                            stdout.write_all(b"\n").unwrap();
                        } else {
                            eprintln!("Parsed value did not match expected type");
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

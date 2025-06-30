use chrono::{NaiveDate, TimeZone, Utc};
use clickhouse_udfs::gb_electricity::datetime_to_sp;
use clickhouse_udfs::row_binary_io::{
    parse_bytes_by_types, serialize_parsed_values, DataType, ParsedValue,
};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let types = [DataType::U32];
    for line in stdin.lock().lines() {
        match line {
            Ok(bytes) => {
                let bytes = bytes.as_bytes();
                match parse_bytes_by_types(bytes, &types) {
                    Ok(parsed) => {
                        if let ParsedValue::U32(ts) = parsed[0] {
                            let dt = Utc.timestamp_opt(ts as i64, 0).unwrap();
                            let (settlement_day, _settlement_period) = datetime_to_sp(dt);
                            // Convert settlement_day to u16 (days since 1970-01-01)
                            let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
                            let settlement_day_u16 = (settlement_day - epoch).num_days() as u16;
                            let output =
                                serialize_parsed_values(&[ParsedValue::U16(settlement_day_u16)]);
                            io::stdout().write_all(&output).unwrap();
                            io::stdout().write_all(b"\n").unwrap();
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

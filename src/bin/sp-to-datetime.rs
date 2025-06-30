use chrono::NaiveDate;
use clickhouse_udfs::gb_electricity::sp_to_datetime;
use clickhouse_udfs::row_binary_io::{
    parse_bytes_by_types, serialize_parsed_values, DataType, ParsedValue,
};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let types = [DataType::U16, DataType::U8];
    for line in stdin.lock().lines() {
        match line {
            Ok(bytes) => {
                let bytes = bytes.as_bytes();
                match parse_bytes_by_types(bytes, &types) {
                    Ok(parsed) => {
                        if let (ParsedValue::U16(day), ParsedValue::U8(period)) =
                            (&parsed[0], &parsed[1])
                        {
                            // Convert u16 to NaiveDate (days since 1970-01-01)
                            let settlement_day = NaiveDate::from_ymd_opt(1970, 1, 1)
                                .unwrap()
                                .succ_opt()
                                .unwrap()
                                .checked_add_days(chrono::Days::new((*day as u64) - 1))
                                .unwrap();
                            let dt = sp_to_datetime(settlement_day, *period);
                            let dt_unix = dt.timestamp() as u32;
                            let output = serialize_parsed_values(&[ParsedValue::U32(dt_unix)]);
                            io::stdout().write_all(&output).unwrap();
                        } else {
                            eprintln!("Parsed values did not match expected types");
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

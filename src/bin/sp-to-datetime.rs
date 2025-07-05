use chrono::NaiveDate;
use clickhouse_udfs::gb_electricity::sp_to_datetime;
use clickhouse_udfs::row_binary_io::{
    parse_bytes_by_types, serialize_parsed_values, DataType, ParsedValue,
};
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let types = [DataType::U16, DataType::U8];
    let mut buffer = vec![0u8; 3];

    loop {
        match stdin.read_exact(&mut buffer) {
            Ok(()) => match parse_bytes_by_types(&buffer, &types) {
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
                        stdout.write_all(&output).unwrap();
                    } else {
                        eprintln!("Parsed values did not match expected types");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(..) => {
                // Exit on EOF or other errors
                break;
            }
        }
    }
}

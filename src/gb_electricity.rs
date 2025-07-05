//! Functions for converting between settlement periods and UTC datetime in the GB electricity market
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use chrono_tz::Europe::London;

pub fn sp_to_datetime(settlement_day: NaiveDate, settlement_period: u8) -> DateTime<Utc> {
    let local_dt = London
        .from_local_datetime(&settlement_day.and_hms_opt(0, 0, 0).unwrap())
        .unwrap();
    let utc_dt = local_dt.with_timezone(&Utc);
    let minutes = 30 * (settlement_period as i64 - 1);
    utc_dt + chrono::Duration::minutes(minutes)
}

pub fn datetime_to_sp(time_stamp: DateTime<Utc>) -> (NaiveDate, u8) {
    let london_time = time_stamp.with_timezone(&London);
    let settlement_day = london_time.date_naive();
    let midnight = London
        .from_local_datetime(&settlement_day.and_hms_opt(0, 0, 0).unwrap())
        .unwrap();
    let duration = london_time.signed_duration_since(midnight);
    let settlement_period = (duration.num_minutes() / 30 + 1) as u8;
    (settlement_day, settlement_period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate, Timelike, Utc};

    #[test]
    fn test_datetime_to_sp_and_sp_to_datetime_roundtrip() {
        let cases = [
            ("2022-01-01T00:00:00+00:00", "2022-01-01", 1),
            ("2022-01-01T00:05:00+00:00", "2022-01-01", 1),
            ("2022-01-01T00:30:00+00:00", "2022-01-01", 2),
            ("2022-01-01T23:30:00+00:00", "2022-01-01", 48),
            ("2022-03-27T00:00:00+00:00", "2022-03-27", 1),
            ("2022-03-27T02:00:00+01:00", "2022-03-27", 3),
            ("2022-03-27T23:59:00+01:00", "2022-03-27", 46),
            ("2022-03-28T00:00:00+01:00", "2022-03-28", 1),
            ("2022-03-28T23:30:00+01:00", "2022-03-28", 48),
            ("2022-10-30T00:00:00+01:00", "2022-10-30", 1),
            ("2022-10-30T01:59:00+01:00", "2022-10-30", 4),
            ("2022-10-30T01:00:00+00:00", "2022-10-30", 5),
            ("2022-10-30T02:00:00+00:00", "2022-10-30", 7),
            ("2022-10-30T23:00:00+00:00", "2022-10-30", 49),
            ("2022-10-30T23:30:00+00:00", "2022-10-30", 50),
            ("2022-10-31T00:00:00+00:00", "2022-10-31", 1),
        ];
        for (ts, expected_date, expected_period) in cases.iter() {
            let dt = DateTime::parse_from_rfc3339(ts)
                .unwrap()
                .with_timezone(&Utc);
            let (settlement_day, settlement_period) = datetime_to_sp(dt);
            assert_eq!(
                settlement_day,
                NaiveDate::parse_from_str(expected_date, "%Y-%m-%d").unwrap(),
                "date for {}",
                ts
            );
            assert_eq!(settlement_period, *expected_period, "period for {}", ts);
            let roundtrip = sp_to_datetime(settlement_day, settlement_period);
            // Check that roundtrip equals ts rounded down to the nearest 30 minutes
            let dt_rounded = dt
                - chrono::Duration::minutes(dt.minute() as i64 % 30)
                - chrono::Duration::seconds(dt.second() as i64)
                - chrono::Duration::nanoseconds(dt.nanosecond() as i64);
            assert_eq!(
                roundtrip, dt_rounded,
                "roundtrip for {}: got {} expected {}",
                ts, roundtrip, dt_rounded
            );
        }
    }
}

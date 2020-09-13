use chrono::prelude::*;
use chrono::Duration;

use crate::enums::Timeframe;

pub(crate) fn calculate_duration_from_timeframe(timeframe: Timeframe) -> (i64, i64) {
    let base_date = Local::now().date();
    let now = Local.ymd(base_date.year(), base_date.month(), base_date.day());
    let now_i64 = now.and_hms(23, 59, 59).timestamp();
    match timeframe {
        Timeframe::Today => (now.and_hms(0, 0, 1).timestamp(), now_i64),
        Timeframe::Yesterday => (
            (now.and_hms(0, 0, 1) - Duration::days(1)).timestamp(),
            (now.and_hms(23, 59, 59) - Duration::days(1)).timestamp(),
        ),
        Timeframe::Week => (
            Local
                .isoywd(base_date.year(), base_date.iso_week().week(), Weekday::Mon)
                .and_hms(0, 0, 1)
                .timestamp(),
            now_i64,
        ),
        Timeframe::LastWeek => {
            let iso_date = base_date - Duration::days(7);
            (
                Local
                    .isoywd(iso_date.year(), iso_date.iso_week().week(), Weekday::Mon)
                    .and_hms(0, 0, 1)
                    .timestamp(),
                Local
                    .isoywd(iso_date.year(), iso_date.iso_week().week(), Weekday::Fri)
                    .and_hms(23, 59, 59)
                    .timestamp(),
            )
        }
        Timeframe::Month => (
            Local
                .ymd(base_date.year(), base_date.month(), 1)
                .and_hms(0, 0, 1)
                .timestamp(),
            now_i64,
        ),
    }
}

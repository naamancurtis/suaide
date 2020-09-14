use chrono::prelude::*;
use chrono::Duration;

use crate::enums::Timeframe;

pub(crate) fn calculate_duration_from_timeframe(
    base_date: Date<Local>,
    timeframe: Timeframe,
) -> (i64, i64) {
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

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref BASE_DATE: Date<Local> = Local.ymd(2000, 3, 7);
    }

    #[test]
    fn today() {
        let min = Local.ymd(2000, 3, 7).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 7).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(*BASE_DATE, Timeframe::Today),
            (min, max)
        );
    }

    #[test]
    fn yesterday() {
        let min = Local.ymd(2000, 3, 6).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 6).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(*BASE_DATE, Timeframe::Yesterday),
            (min, max)
        );
    }

    #[test]
    fn yesterday_over_month() {
        let min = Local.ymd(2000, 2, 29).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 2, 29).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(BASE_DATE.with_day(1).unwrap(), Timeframe::Yesterday),
            (min, max)
        );
    }

    #[test]
    fn yesterday_over_year() {
        let today = Local.ymd(2000, 1, 1);
        let min = Local.ymd(1999, 12, 31).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(1999, 12, 31).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::Yesterday),
            (min, max)
        );
    }

    #[test]
    fn week_monday() {
        let today = BASE_DATE.pred();
        let min = BASE_DATE.with_day(6).unwrap().and_hms(0, 0, 1).timestamp();
        let max = today.and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::Week),
            (min, max)
        );
    }

    #[test]
    fn week_tuesday() {
        let today = &BASE_DATE;
        let min = BASE_DATE.with_day(6).unwrap().and_hms(0, 0, 1).timestamp();
        let max = today.and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(**today, Timeframe::Week),
            (min, max)
        );
    }

    #[test]
    fn week_wednesday() {
        let today = BASE_DATE.succ();
        let min = BASE_DATE.with_day(6).unwrap().and_hms(0, 0, 1).timestamp();
        let max = today.and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::Week),
            (min, max)
        );
    }

    #[test]
    fn week_thursday() {
        let today = BASE_DATE.succ().succ();
        let min = BASE_DATE.with_day(6).unwrap().and_hms(0, 0, 1).timestamp();
        let max = today.and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::Week),
            (min, max)
        );
    }

    #[test]
    fn week_friday() {
        let today = BASE_DATE.succ().succ().succ();
        let min = BASE_DATE.with_day(6).unwrap().and_hms(0, 0, 1).timestamp();
        let max = today.and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::Week),
            (min, max)
        );
    }

    #[test]
    fn week_saturday() {
        let today = BASE_DATE.succ().succ().succ().succ();
        let min = BASE_DATE.with_day(6).unwrap().and_hms(0, 0, 1).timestamp();
        let max = today.and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::Week),
            (min, max)
        );
    }

    #[test]
    fn week_sunday() {
        let today = BASE_DATE.succ().succ().succ().succ().succ();
        let min = BASE_DATE.with_day(6).unwrap().and_hms(0, 0, 1).timestamp();
        let max = today.and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::Week),
            (min, max)
        );
    }

    #[test]
    fn last_week_monday() {
        let today = BASE_DATE.pred();
        let min = Local.ymd(2000, 2, 28).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 3).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::LastWeek),
            (min, max)
        );
    }

    #[test]
    fn last_week_tuesday() {
        let today = &BASE_DATE;
        let min = Local.ymd(2000, 2, 28).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 3).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(**today, Timeframe::LastWeek),
            (min, max)
        );
    }

    #[test]
    fn last_week_wednesday() {
        let today = BASE_DATE.succ();
        let min = Local.ymd(2000, 2, 28).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 3).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::LastWeek),
            (min, max)
        );
    }

    #[test]
    fn last_week_thursday() {
        let today = BASE_DATE.succ().succ();
        let min = Local.ymd(2000, 2, 28).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 3).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::LastWeek),
            (min, max)
        );
    }

    #[test]
    fn last_week_friday() {
        let today = BASE_DATE.succ().succ().succ();
        let min = Local.ymd(2000, 2, 28).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 3).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::LastWeek),
            (min, max)
        );
    }

    #[test]
    fn last_week_saturday() {
        let today = BASE_DATE.succ().succ().succ().succ();
        let min = Local.ymd(2000, 2, 28).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 3).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::LastWeek),
            (min, max)
        );
    }

    #[test]
    fn last_week_sunday() {
        let today = BASE_DATE.succ().succ().succ().succ().succ();
        let min = Local.ymd(2000, 2, 28).and_hms(0, 0, 1).timestamp();
        let max = Local.ymd(2000, 3, 3).and_hms(23, 59, 59).timestamp();
        assert_eq!(
            calculate_duration_from_timeframe(today, Timeframe::LastWeek),
            (min, max)
        );
    }
}

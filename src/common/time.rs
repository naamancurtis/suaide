use chrono::prelude::*;
use chrono::Duration;

use crate::{
    common::{DATE_INPUT_LONG, DATE_INPUT_SHORT},
    domain::{SuaideError, Timeframe},
};

pub(crate) fn calculate_duration_from_dates(
    from: &str,
    to: &str,
) -> Result<(i64, i64), SuaideError> {
    let from = match NaiveDate::parse_from_str(from, DATE_INPUT_SHORT) {
        Ok(r) => r,
        Err(_) => NaiveDate::parse_from_str(from, DATE_INPUT_LONG)?,
    };
    let to = match NaiveDate::parse_from_str(to, DATE_INPUT_SHORT) {
        Ok(r) => r,
        Err(_) => NaiveDate::parse_from_str(to, DATE_INPUT_LONG)?,
    };
    let from = Local
        .ymd(from.year(), from.month(), from.day())
        .and_hms(0, 0, 1)
        .timestamp();
    let to = Local
        .ymd(to.year(), to.month(), to.day())
        .and_hms(23, 59, 59)
        .timestamp();
    Ok((from, to))
}

pub(crate) fn calculate_duration_from_timeframe(
    base_date: Date<Local>,
    timeframe: Timeframe,
) -> (i64, i64) {
    let base = Local.ymd(base_date.year(), base_date.month(), base_date.day());
    let base_hms = base.and_hms(23, 59, 59);

    let (start, end) = match timeframe {
        Timeframe::Today => (base.and_hms(0, 0, 1), base_hms),

        Timeframe::Yesterday => (
            base.and_hms(0, 0, 1) - Duration::days(1),
            base.and_hms(23, 59, 59) - Duration::days(1),
        ),

        Timeframe::Week => (
            Local
                .isoywd(base_date.year(), base_date.iso_week().week(), Weekday::Mon)
                .and_hms(0, 0, 1),
            base_hms,
        ),

        Timeframe::LastWeek => {
            let iso_date = base_date - Duration::days(7);
            (
                Local
                    .isoywd(iso_date.year(), iso_date.iso_week().week(), Weekday::Mon)
                    .and_hms(0, 0, 1),
                Local
                    .isoywd(iso_date.year(), iso_date.iso_week().week(), Weekday::Fri)
                    .and_hms(23, 59, 59),
            )
        }

        Timeframe::Month => (
            Local
                .ymd(base_date.year(), base_date.month(), 1)
                .and_hms(0, 0, 1),
            base_hms,
        ),
    };

    (start.timestamp(), end.timestamp())
}

#[cfg(test)]
mod test_from_dates {
    use super::*;

    #[test]
    fn parses_from_short_input() {
        let from = "2020-01-23";
        let to = "2019-11-01";

        let result = calculate_duration_from_dates(from, to);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result.0,
            Local.ymd(2020, 1, 23).and_hms(0, 0, 1).timestamp()
        );
        assert_eq!(
            result.1,
            Local.ymd(2019, 11, 1).and_hms(23, 59, 59).timestamp()
        )
    }

    #[test]
    fn parses_from_long_input() {
        let from = "23 Jan 2020";
        let to = "1 Nov 2019";

        let result = calculate_duration_from_dates(from, to);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result.0,
            Local.ymd(2020, 1, 23).and_hms(0, 0, 1).timestamp()
        );
        assert_eq!(
            result.1,
            Local.ymd(2019, 11, 1).and_hms(23, 59, 59).timestamp()
        )
    }

    #[test]
    fn parses_from_long_input_with_full_months() {
        let from = "23 January 2020";
        let to = "1 November 2019";

        let result = calculate_duration_from_dates(from, to);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result.0,
            Local.ymd(2020, 1, 23).and_hms(0, 0, 1).timestamp()
        );
        assert_eq!(
            result.1,
            Local.ymd(2019, 11, 1).and_hms(23, 59, 59).timestamp()
        )
    }
}

#[cfg(test)]
mod test_from_timeframe {
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

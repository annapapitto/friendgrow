use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};

const MAX_FREQ_WEEKS: i32 = 52;
const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn parse_date(date: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date, DATE_FORMAT).context(format!(
        "Date {} does not have format {}",
        date, DATE_FORMAT
    ))
}

pub fn check_frequency(freq_weeks: i32) -> Result<()> {
    if freq_weeks <= 0 || freq_weeks > MAX_FREQ_WEEKS {
        return Err(anyhow::anyhow!(
            "Must see friends between every 1 week and every {} weeks",
            MAX_FREQ_WEEKS
        ));
    }
    Ok(())
}

pub fn check_new_seen(new_date: NaiveDate, last_date: Option<String>) -> Result<()> {
    if last_date.is_some() {
        let last_date = parse_date(&last_date.unwrap())?;
        if last_date > new_date {
            return Err(anyhow::anyhow!(
                "Already seen more recently on {}",
                last_date
            ));
        }
    }

    if new_date > local_today() {
        return Err(anyhow::anyhow!("Cannot record in the future"));
    }
    Ok(())
}

pub fn local_today() -> NaiveDate {
    Local::now().date().naive_local()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_parse_date() {
        let correct = NaiveDate::from_ymd(2021, 10, 26);
        let res = parse_date("2021-10-26");
        assert_eq!(res.unwrap(), correct);

        let correct = NaiveDate::from_ymd(100, 2, 3);
        let res = parse_date("100-2-3");
        assert_eq!(res.unwrap(), correct);
        let res = parse_date("0100-02-03");
        assert_eq!(res.unwrap(), correct);
    }

    #[test]
    fn test_parse_date_errors() {
        assert!(parse_date("").is_err());
    }

    #[test]
    fn test_parse_date_month_day_flipped() {
        assert!(parse_date("2021-20-10").is_err());
    }

    #[test]
    fn test_check_frequency() {
        let check = check_frequency(1);
        assert!(check.is_ok());
        let check = check_frequency(20);
        assert!(check.is_ok());
        let check = check_frequency(52);
        assert!(check.is_ok());
    }

    #[test]
    fn test_check_frequency_zero() {
        let check = check_frequency(0);
        assert!(check.is_err());
    }

    #[test]
    fn test_check_frequency_negative() {
        let check = check_frequency(-1);
        assert!(check.is_err());
    }

    #[test]
    fn test_check_frequency_big() {
        let check = check_frequency(53);
        assert!(check.is_err());
    }

    #[test]
    fn test_check_new_seen() {
        let new_date = NaiveDate::from_ymd(102, 2, 5);
        let check = check_new_seen(new_date, Some("100-2-4".to_string()));
        assert!(check.is_ok());
        let check = check_new_seen(new_date, Some("101-12-3".to_string()));
        assert!(check.is_ok());
        let check = check_new_seen(new_date, None);
        assert!(check.is_ok());
    }

    #[test]
    fn test_check_new_seen_earlier() {
        let new_date = NaiveDate::from_ymd(200, 4, 7);
        let check = check_new_seen(new_date, Some("200-4-8".to_string()));
        assert!(check.is_err());
    }

    #[test]
    fn test_check_new_seen_future() {
        let tomorrow = local_today() + Duration::days(1);
        let check = check_new_seen(tomorrow, None);
        assert!(check.is_err());
    }
}

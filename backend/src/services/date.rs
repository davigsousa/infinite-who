use crate::schemas::date::Date;
use chrono::Datelike;

pub fn get_current_date() -> Date {
    let current_utc = chrono::Utc::now();
    let year = current_utc.year();
    let month = current_utc.month();
    let day = current_utc.day();

    Date { day, month, year }
}

pub fn date_plus_month(date: Date) -> Date {
    let option_date = chrono::NaiveDate::from_ymd_opt(date.year, date.month, date.day);

    let chrono_date = match option_date {
        Some(naive_date) => naive_date,
        None => return date,
    };

    let chrono_date = chrono_date
        .checked_add_signed(chrono::Duration::days(30))
        .expect("Invalid date");

    Date {
        day: chrono_date.day(),
        month: chrono_date.month(),
        year: chrono_date.year(),
    }
}

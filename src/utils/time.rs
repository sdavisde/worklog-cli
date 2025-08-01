use chrono::{Days, Local};

pub fn get_today_date() -> String {
    return Local::now().format("%Y-%m-%d").to_string();
}

pub fn get_yesterday_date() -> String {
    return Local::now().checked_sub_days(Days::new(1)).unwrap().format("%Y-%m-%d").to_string();
}

use chrono::Local;

pub fn get_today_date() -> String {
    return Local::now().format("%Y-%m-%d").to_string();
}

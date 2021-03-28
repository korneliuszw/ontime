use chrono::Weekday;
use std::borrow::Cow;

// Chrono weekday name to it's full name
pub fn chrono_to_string(weekday: &Weekday) -> Cow<'static, str> {
    match weekday {
        Weekday::Mon => "monday",
        Weekday::Tue => "tuesday",
        Weekday::Wed => "wednesday",
        Weekday::Thu => "thursday",
        Weekday::Fri => "friday",
        Weekday::Sat => "saturday",
        Weekday::Sun => "sunday",
    }
    .into()
}

use chrono::{DateTime, Locale, Utc};
use std::{convert::TryInto, env::var};

#[must_use]
pub fn language() -> String {
    var("LANGUAGE").unwrap()
}

fn time_locale_from_env() -> Locale {
    language().as_str().try_into().unwrap_or(Locale::POSIX)
}

#[must_use]
pub fn format(date_time: DateTime<Utc>) -> String {
    date_time
        .format_localized("%c", time_locale_from_env())
        .to_string()
}

use chrono::DateTime;
use chrono::Locale;
use chrono::Utc;
use std::convert::TryInto;

fn time_locale_from_env() -> Locale {
    std::env::var("LANGUAGE")
        .map(|language| language.as_str().try_into())
        .unwrap_or(Ok(Locale::POSIX))
        .unwrap()
}

#[must_use]
pub fn format(date_time: DateTime<Utc>) -> String {
    date_time
        .format_localized("%c", time_locale_from_env())
        .to_string()
}

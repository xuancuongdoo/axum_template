mod error;

pub use self::error::{Error, Result};

use base64::enginne::{general_purpose, Engine};
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: &OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap()
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339)
        .map_err(|_| Error::DataFailedToParse(moment.to_string()))
}

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::STANDARD.encode(content.as_ref())
}

use crate::Props;
use chrono::{DateTime, FixedOffset, Utc};

pub mod body;
pub mod head;

pub trait Component {
    fn write(&self, props: &Props, buf: &mut String);
}

pub fn escape_html(s: &str, buf: &mut String) {
    for c in s.chars() {
        match c {
            '&' => buf.push_str("&amp;"),
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '"' => buf.push_str("&quot;"),
            _ => buf.push(c),
        }
    }
}

pub fn replace_slash(s: &str, buf: &mut String) {
    for c in s.chars() {
        match c {
            '-' => buf.push('/'),
            _ => buf.push(c),
        }
    }
}

/// UTCからJSTに変換
pub fn utc_to_jst_time(timestamp: &DateTime<Utc>, buf: &mut String) {
    let jst: DateTime<FixedOffset> =
        timestamp.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
    *buf += &jst.format("%Y/%m/%d %H:%M").to_string();
}

/// UTCからJSTに変換
pub fn utc_to_jst_date(timestamp: &DateTime<Utc>, buf: &mut String) {
    let jst: DateTime<FixedOffset> =
        timestamp.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
    *buf += &jst.format("%Y/%m/%d").to_string();
}

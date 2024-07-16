/// 指定された期間の日本の祝日を文字列にして返す
pub fn get_holiday(gantt_start: &Option<String>, gantt_end: &Option<String>) -> String {
    let (start, end) = match (gantt_start, gantt_end) {
        (Some(start), Some(end)) => (start, end),
        _ => return String::from(""),
    };

    if start.len() != 10 || end.len() != 10 {
        return String::from("");
    }

    let holidays = crate::HOLIDAYS.get().unwrap();
    let mut buf = String::new();

    for holiday in holidays {
        if **start > **holiday {
            continue;
        }
        if **holiday > **end {
            break;
        }
        if !buf.is_empty() {
            buf += ",";
        }
        buf += holiday;
    }

    buf
}

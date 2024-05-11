use crate::Props;

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

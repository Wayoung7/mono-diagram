use std::iter::repeat;

use unicode_width::UnicodeWidthStr;

pub fn pad_string_center(s: &str, width: usize, l_pad: char, r_pad: char) -> String {
    if UnicodeWidthStr::width(s) > width {
        s.to_owned()
    } else {
        let total_pad_len = width - UnicodeWidthStr::width(s);
        let l_pad_len = total_pad_len / 2;
        let r_pad_len = total_pad_len - l_pad_len;
        format!(
            "{}{}{}",
            repeat(l_pad).take(l_pad_len).collect::<String>(),
            s,
            repeat(r_pad).take(r_pad_len).collect::<String>()
        )
    }
}

pub fn pad_string_right(s: &str, width: usize, r_pad: char) -> String {
    if UnicodeWidthStr::width(s) > width {
        s.to_owned()
    } else {
        let pad_len = width - UnicodeWidthStr::width(s);
        format!("{}{}", s, repeat(r_pad).take(pad_len).collect::<String>())
    }
}

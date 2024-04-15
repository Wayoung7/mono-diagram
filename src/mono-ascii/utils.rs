use std::iter::repeat;

pub fn pad_string_center(s: &String, width: usize, l_pad: char, r_pad: char) -> String {
    if s.len() > width {
        s.to_owned()
    } else {
        let total_pad_len = width - s.len();
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

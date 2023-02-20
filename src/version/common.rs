pub fn parse_num_id(s: &str) -> Option<(u32, &str)> {
    let cnt = s.chars().take_while(|c| c.is_ascii_digit()).count();

    if cnt == 0 || (cnt > 1 && s.starts_with('0')) {
        return None;
    }

    s[..cnt].parse().ok().map(|id| (id, &s[cnt..]))
}

#[test]
fn test_parse_num_id() {
    assert_eq!(None, parse_num_id(""));
    assert_eq!(None, parse_num_id("a1"));
    assert_eq!(None, parse_num_id("01"));
    assert_eq!(Some((0, "")), parse_num_id("0"));
    assert_eq!(Some((102, ".")), parse_num_id("102."));
}

pub fn parse_dot_sep_list(s: &str, check_part: fn(&str) -> bool) -> Option<(Vec<String>, &str)> {
    let mut r = s;
    let mut items = Vec::new();

    loop {
        let cnt = r
            .chars()
            .take_while(|c| *c == '-' || c.is_ascii_alphanumeric())
            .count();

        if cnt == 0 || !check_part(&r[..cnt]) {
            return None;
        }

        items.push(String::from(&r[..cnt]));
        r = &r[cnt..];

        if let Some(t) = r.strip_prefix('.') {
            r = t;
        } else {
            break;
        }
    }

    if !items.is_empty() {
        Some((items, r))
    } else {
        None
    }
}

#[test]
fn test_parse_dot_sep_list() {
    assert_eq!(None, parse_dot_sep_list("", |_| true));
    assert_eq!(None, parse_dot_sep_list(".foo", |_| true));
    assert_eq!(None, parse_dot_sep_list("foo.", |_| true));
    assert_eq!(None, parse_dot_sep_list("foo..bar", |_| true));
    assert_eq!(None, parse_dot_sep_list("foo.bar", |p| p != "bar"));
    assert_eq!(None, parse_dot_sep_list("foo.💩.bar", |_| true));
    assert_eq!(
        Some((vec!["foo".to_string(), "01".to_string()], "")),
        parse_dot_sep_list("foo.01", |_| true)
    );
    assert_eq!(
        Some((vec!["a".to_string()], "💩.b")),
        parse_dot_sep_list("a💩.b", |_| true)
    );
    assert_eq!(
        Some((vec!["-Ab1".to_string()], "_")),
        parse_dot_sep_list("-Ab1_", |_| true)
    );
}

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

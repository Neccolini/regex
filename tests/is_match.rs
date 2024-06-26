use regex::Regex;

// basic tests
#[test]
fn case0() {
    let re = Regex::new(r"(a|b)*").unwrap();

    assert!(re.is_match("aa"));
    assert!(re.is_match(""));
    assert!(re.is_match("bbb"));
    assert!(re.is_match("ab"));
    assert!(!re.is_match("abc"));
}

#[test]
fn case1() {
    let re = Regex::new(r"ab|cd").unwrap();

    assert!(re.is_match("ab"));
    assert!(re.is_match("cd"));
    assert!(!re.is_match("ac"));
    assert!(!re.is_match("bd"));
}

#[test]
fn case2() {
    let re = Regex::new(r"a(b|c)d").unwrap();

    assert!(re.is_match("abd"));
    assert!(re.is_match("acd"));
    assert!(!re.is_match("ad"));
    assert!(!re.is_match("abcd"));
}

#[test]
fn case3() {
    let re = Regex::new(r"a(b|c)*d").unwrap();

    assert!(re.is_match("ad"));
    assert!(re.is_match("abd"));
    assert!(re.is_match("acd"));
    assert!(re.is_match("abcbcd"));
    assert!(!re.is_match("abc"));
    assert!(!re.is_match("aabcd"));
}

#[test]
fn case4() {
    let re = Regex::new(r"a(b|c)?d").unwrap();

    assert!(re.is_match("ad"));
    assert!(re.is_match("abd"));
    assert!(re.is_match("acd"));
    assert!(!re.is_match("abcbcd"));
    assert!(!re.is_match("abc"));
    assert!(!re.is_match("abcd"));
}

#[test]
fn case5() {
    let re = Regex::new(r"a(b|c)+d").unwrap();

    assert!(re.is_match("abd"));
    assert!(re.is_match("acd"));
    assert!(re.is_match("abcd"));
    assert!(re.is_match("abcbcd"));
    assert!(!re.is_match("ad"));
    assert!(!re.is_match("abc"));
    assert!(!re.is_match("babcbd"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use escape_string;
    use regex::Regex;

    #[test]
    fn escape() {
        let s = "\x02pp\x02rp\x02rp\x02py\x02ry\x02ry\x02pt\x02rt\x02rt\x02ph\x02rh\x02po\x02ro\x02rh\x02ro\x02pn\x02rn\x02rn\x02p3\x02r3\x02r3\x02p\x00";
        let char_vec: Vec<char> = s.chars().collect();
        println!("{:?}", char_vec);
    }
    
}

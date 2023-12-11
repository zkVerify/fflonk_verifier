use substrate_bn::arith::U256;

macro_rules! u256 {
    ($s:literal) => {{
        const STRING: &'static [u8] = $s.as_bytes();
        $crate::macros::decode(STRING)
    }};
}
pub(crate) use u256;

#[cfg(test)]
macro_rules! fr {
    ($s:literal) => {{
        use $crate::macros::u256;
        u256!($s).into_fr()
    }};
}
#[cfg(test)]
pub(crate) use fr;

#[cfg(test)]
macro_rules! u256s {
    ($($s:literal),*) => {{
        use $crate::macros::u256;

        [$(u256!($s)),*]
    }};
    ($($s:literal,)*) => {
        u256s![$($s),*]
    };
}
#[cfg(test)]
pub(crate) use u256s;

#[cfg(test)]
macro_rules! g1 {
    ($s1:literal, $s2:literal) => {{
        use $crate::macros::u256;
        AffineG1::new(
            Fq::from_u256(u256!($s1)).unwrap(),
            Fq::from_u256(u256!($s2)).unwrap(),
        )
        .unwrap()
        .into()
    }};
}
#[cfg(test)]
pub(crate) use g1;

const fn next_hex_char(string: &[u8], mut pos: usize) -> Option<(u8, usize)> {
    while pos < string.len() {
        let raw_val = string[pos];
        pos += 1;
        let val = match raw_val {
            b'0'..=b'9' => raw_val - 48,
            b'A'..=b'F' => raw_val - 55,
            b'a'..=b'f' => raw_val - 87,
            b' ' | b'\r' | b'\n' | b'\t' => continue,
            0..=127 => panic!("Encountered invalid ASCII character"),
            _ => panic!("Encountered non-ASCII character"),
        };
        return Some((val, pos));
    }
    None
}

const fn next_byte(string: &[u8], pos: usize) -> Option<(u8, usize)> {
    let (half1, pos) = match next_hex_char(string, pos) {
        Some(v) => v,
        None => return None,
    };
    let (half2, pos) = match next_hex_char(string, pos) {
        Some(v) => v,
        None => panic!("Odd number of hex characters"),
    };
    Some(((half1 << 4) + half2, pos))
}

pub(crate) const fn decode(string: &[u8]) -> U256 {
    let mut buf = [0u128; 2];
    let mut buf_pos = 1;
    let mut pos = 0;
    let mut bytes = 0;
    while let Some((byte, new_pos)) = next_byte(string, pos) {
        if bytes == 16 {
            bytes = 0;
            buf_pos -= 1;
        }
        buf[buf_pos] = buf[buf_pos] << 8 | byte as u128;
        bytes += 1;
        pos = new_pos;
    }
    assert!(
        bytes == 16 && buf_pos == 0,
        "You should provide exactly 32 bytes hex"
    );
    U256(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode_u256_correctly() {
        let expected = U256::from_slice(&hex_literal::hex!(
            "2bb635ee7d9e1790de1d6ccec2d1e13dec5c4beffd75d71520107c791857c45e"
        ))
        .unwrap();
        assert_eq!(
            expected,
            u256!("2bb635ee7d9e1790de1d6ccec2d1e13dec5c4beffd75d71520107c791857c45e")
        );
    }
}

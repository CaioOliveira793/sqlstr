// A lookup table to prevent the need for conditional branching
// The value of the remainder of each step will be used as the index
const LOOKUP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// A lookup table optimized for decimal lookups. Each two indices represents one possible number.
const DEC_LOOKUP: &[u8; 200] = b"0001020304050607080910111213141516171819\
                                 2021222324252627282930313233343536373839\
                                 4041424344454647484950515253545556575859\
                                 6061626364656667686970717273747576777879\
                                 8081828384858687888990919293949596979899";

/// Formats the u32 number into the `buf` and return a reference from the written buffer.
pub fn format_u32_base10(mut number: u32, buf: &mut [u8; 10]) -> &str {
    let mut index = buf.len() - 1;
    if number == 0 {
        buf[index] = b'0';
        // SAFETY:
        // The sliced `buf` contains a valid `'0'` character
        return unsafe { core::str::from_utf8_unchecked(&buf[index..]) };
    }

    // Convert using optimized base 10 algorithm
    while number > 9999 {
        let rem = (number % 10000) as u16;
        let (frst, scnd) = ((rem / 100) * 2, (rem % 100) * 2);
        buf[index - 3..index - 1].copy_from_slice(&DEC_LOOKUP[frst as usize..frst as usize + 2]);
        buf[index - 1..index + 1].copy_from_slice(&DEC_LOOKUP[scnd as usize..scnd as usize + 2]);
        index = index.wrapping_sub(4);
        number /= 10000;
    }

    if number > 999 {
        let (frst, scnd) = ((number / 100) * 2, (number % 100) * 2);
        buf[index - 3..index - 1].copy_from_slice(&DEC_LOOKUP[frst as usize..frst as usize + 2]);
        buf[index - 1..index + 1].copy_from_slice(&DEC_LOOKUP[scnd as usize..scnd as usize + 2]);
        index = index.wrapping_sub(4);
    } else if number > 99 {
        let section = (number as u16 / 10) * 2;
        buf[index - 2..index].copy_from_slice(&DEC_LOOKUP[section as usize..section as usize + 2]);
        buf[index] = LOOKUP[(number % 10) as usize];
        index = index.wrapping_sub(3);
    } else if number > 9 {
        number *= 2;
        buf[index - 1..index + 1]
            .copy_from_slice(&DEC_LOOKUP[number as usize..number as usize + 2]);
        index = index.wrapping_sub(2);
    } else {
        buf[index] = LOOKUP[number as usize];
        index = index.wrapping_sub(1);
    }

    // SAFETY:
    // The sliced `buf` contains a formatted positive number (`u32`) represented
    // through (`b'0'..=b'9'`)
    unsafe { core::str::from_utf8_unchecked(&buf[index.wrapping_add(1)..]) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u32_zero() {
        let mut buf = [0; 10];
        let formated = format_u32_base10(0, &mut buf);

        assert_eq!(formated, "0");
        assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 0, 0, b'0']);
    }

    #[test]
    fn u32_max() {
        let mut buf = [0; 10];
        let formated = format_u32_base10(u32::MAX, &mut buf);

        assert_eq!(formated, "4294967295");
        assert_eq!(
            buf,
            [b'4', b'2', b'9', b'4', b'9', b'6', b'7', b'2', b'9', b'5']
        );
    }

    #[test]
    fn u32_inputs() {
        let mut buf = [0; 10];
        let formated = format_u32_base10(8293742, &mut buf);

        assert_eq!(formated, "8293742");
        assert_eq!(buf, [0, 0, 0, b'8', b'2', b'9', b'3', b'7', b'4', b'2']);
    }
}

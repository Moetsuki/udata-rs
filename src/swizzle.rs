#![allow(dead_code)]
#![allow(unused_imports)]
use base64::prelude::*;
use pretty_hex::*;

pub fn base64_swizzle(input: &[u8]) -> String {
    let mut output = input.to_vec();

    // If input is odd length, we need to add a padding byte
    if input.len() % 2 == 1 {
        output.push(output[0]);
    }

    for pair in output.chunks_mut(2) {
        if let [x, y] = pair {
            let n1 = *x % 8;

            *y = y.rotate_right(n1.into());

            let n2 = *y % 8;

            *x = x.rotate_right(n2.into());
        }
    }

    BASE64_STANDARD.encode(&output)
}

pub fn base64_unswizzle(input: &str) -> Vec<u8> {
    let mut output = BASE64_STANDARD.decode(input).unwrap();

    // If input is odd length, we need to add a padding byte
    if input.len() % 2 == 1 {
        output.push(output[0]);
    }

    for pair in output.chunks_mut(2) {
        if let [x, y] = pair {
            let n1 = *y % 8;

            *x = x.rotate_left(n1.into());

            let n2 = *x % 8;

            *y = y.rotate_left(n2.into());
        }
    }

    output.to_vec()
}

pub fn swizzle(input: &[u8]) -> Vec<u8> {
    let mut output = input.to_vec();

    // If input is odd length, we need to add a padding byte
    if input.len() % 2 == 1 {
        output.push(output[0]);
    }

    for pair in output.chunks_mut(2) {
        if let [x, y] = pair {
            let n1 = *x % 8;

            *y = y.rotate_right(n1.into());

            let n2 = *y % 8;

            *x = x.rotate_right(n2.into());
        }
    }

    output.to_vec()
}

pub fn unswizzle(input: &[u8]) -> Vec<u8> {
    let mut output = input.to_vec();
    // If input is odd length, we need to add a padding byte
    if input.len() % 2 == 1 {
        output.push(output[0]);
    }

    for pair in output.chunks_mut(2) {
        if let [x, y] = pair {
            let n1 = *y % 8;

            *x = x.rotate_left(n1.into());

            let n2 = *x % 8;

            *y = y.rotate_left(n2.into());
        }
    }

    output.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swizzle() {
        let x0: u8 = 0b0001_0101;
        let y0 : u8 = 0b1011_1100;

        let mut x: u8 = 0b0001_0101;
        let mut y: u8 = 0b1011_1100;

        // swizzle
        {
            let n1 = x % 8;

            y = (y >> n1) | (y << (8 - n1));

            let n2 = y % 8;

            x = (x >> n2) | (x << (8 - n2));
        }
        
        // unswizzle
        {
            let n1 = y % 8;

            x = (x << n1) | (x >> (8 - n1));

            let n2 = x % 8;

            y = (y << n2) | (y >> (8 - n2));
        }

        assert_eq!(x, x0);
        assert_eq!(y, y0);

        let data = vec![0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8];
        
        let swizzled = swizzle(&data);
        let unswizzled = unswizzle(&swizzled);

        assert_eq!(data, unswizzled);
    }

    #[test]
    fn test_base64_swizzle() {
        let data = "A quick brown fox jumps over the lazy dog!";
        let swizzled = base64_swizzle(data.as_bytes());
        let unswizzled = base64_unswizzle(&swizzled);
        assert_eq!(data.as_bytes(), unswizzled.as_slice());
        assert_eq!(data.len(), unswizzled.len());
        assert_eq!(data, String::from_utf8(unswizzled.clone()).unwrap());

        let udata = BASE64_STANDARD.decode(&swizzled).unwrap();
        assert_ne!(data.as_bytes(), udata.as_slice());
        eprintln!("{:?}\n", udata.as_slice().hex_dump());
        eprintln!("{:?}\n", unswizzled.as_slice().hex_dump());
    }
}
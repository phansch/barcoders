//! Encoder for Code11 (USD-8) barcodes.
//!
//! Code11 is able to encode all of the decimal digits and the dash character. It is mainly
//! used in the telecommunications industry.
//!
//! Code11 is a discrete symbology.

use sym::{Parse, helpers};
use error::Result;
use std::ops::Range;

// Character -> Binary mappings for each of the allowable characters.
// The special "full-ASCII" characters are represented with (, ), [, ].
const CHARS: [(char, &[u8]); 11] = [
    ('0', &[1,0,1,0,1,1]), ('1', &[1,1,0,1,0,1,1]), ('2', &[1,0,0,1,0,1,1]),
    ('3', &[1,1,0,0,1,0,1]), ('4', &[1,0,1,1,0,1,1]), ('5', &[1,1,0,1,1,0,1]),
    ('6', &[1,0,0,1,1,0,1]), ('7', &[1,0,1,0,0,1,1]), ('8', &[1,1,0,1,0,0,1]),
    ('9', &[1,1,0,1,0,1]), ('-', &[1,0,1,1,0,1]),
];

// Code11 barcodes must start and end with a special character.
const GUARD: [u8; 7] = [1,0,1,1,0,0,1];
const SEPARATOR: [u8; 1] = [0];

/// The Code11 barcode type.
#[derive(Debug)]
pub struct Code11(Vec<char>);

/// The USD-8 barcode type.
pub type USD8 = Code11;

impl Code11 {
    /// Creates a new barcode.
    /// Returns Result<Code11, Error> indicating parse success.
    pub fn new<T: AsRef<str>>(data: T) -> Result<Code11> {
        Code11::parse(data.as_ref()).and_then(|d| {
            Ok(Code11(d.chars()
                       .collect()))
        })
    }

    fn char_encoding(&self, c: char) -> &[u8] {
        match CHARS.iter().find(|&ch| ch.0 == c) {
            Some(&(_, enc)) => enc,
            None => panic!(format!("Unknown char: {}", c)),
        }
    }

    /// Calculates a checksum character using a weighted modulo-11 algorithm.
    fn checksum_char(&self, data: &Vec<char>, modulo: usize, weight_threshold: usize) -> Option<char> {
        let get_char_pos = |&c| CHARS.iter().position(|t| t.0 == c).unwrap();
        let weight = |i| {
            match i % weight_threshold {
                0 => weight_threshold,
                n => n,
            }
        };
        let positions = data.iter().map(&get_char_pos);
        let index = positions.rev()
                             .enumerate()
                             .fold(0, |acc, (i, pos)| acc + (weight(i + 1) * pos));

        match CHARS.get(index % modulo) {
            Some(&(c, _)) => Some(c),
            None => None,
        }
    }


    /// Calculates the C checksum character using a weighted modulo-11 algorithm.
    pub fn c_checksum_char(&self) -> Option<char> {
        self.checksum_char(&self.0, 11, 10)
    }

    /// Calculates the K checksum character using a weighted modulo-9 algorithm.
    pub fn k_checksum_char(&self, c_checksum: char) -> Option<char> {
        let mut data: Vec<char> = self.0.clone();
        data.push(c_checksum);

        self.checksum_char(&data, 9, 9)
    }

    fn push_encoding(&self, into: &mut Vec<u8>, from: &[u8]) {
        into.extend(from.iter().cloned());
        into.extend(&[0]);
    }

    fn payload(&self) -> Vec<u8> {
        let mut enc = vec![];
        let c_checksum = self.c_checksum_char().expect("Cannot compute checksum C");

        for &c in &self.0 {
            self.push_encoding(&mut enc, self.char_encoding(c));
        }

        self.push_encoding(&mut enc, self.char_encoding(c_checksum));

        // K-checksum is only appended on barcodes greater than 10 characters.
        //if self.0.len() > 10 {
            let k_checksum = self.k_checksum_char(c_checksum).expect("Cannot compute checksum K");

            self.push_encoding(&mut enc, self.char_encoding(k_checksum));
        //}

        enc
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of encoded binary digits.
    pub fn encode(&self) -> Vec<u8> {
        let guard = &GUARD[..];

        helpers::join_slices(&[guard, &[0],
                             &self.payload()[..],
                             guard][..])
    }
}

impl Parse for Code11 {
    /// Returns the valid length of data acceptable in this type of barcode.
    /// Code11 barcodes are variable-length.
    fn valid_len() -> Range<u32> {
        1..256
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        let (chars, _): (Vec<_>, Vec<_>) = CHARS.iter().cloned().unzip();
        chars
    }
}

#[cfg(test)]
mod tests {
    use sym::code11::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn invalid_length_code11() {
        let code11 = Code11::new("".to_owned());

        assert_eq!(code11.err().unwrap(), Error::Length);
    }

    #[test]
    fn invalid_data_code11() {
        let code11 = Code11::new("NOTDIGITS".to_owned());

        assert_eq!(code11.err().unwrap(), Error::Character);
    }

    #[test]
    fn code11_encode() {
        let code111 = Code11::new("123-45".to_owned()).unwrap();
        let code112 = Code11::new("666".to_owned()).unwrap();

        assert_eq!(collapse_vec(code111.encode()), "101100101101011010010110110010101011010101101101101101011011010100101101011001".to_owned());
        assert_eq!(collapse_vec(code112.encode()), "1011001010011010100110101001101011001010110010101011001".to_owned());
    }
}

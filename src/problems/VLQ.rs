#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}


const MASK: u32 = 0x7F;
const ODD_BIT: u32 = 0x80;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
///
/// Формирование байтов:
// Для старшей группы (0000001) устанавливаем флаг продолжения в 1: 10000001 (0x81)
// Для младшей группы (0001001) флаг продолжения равен 0: 00001001 (0x09)
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = vec![];

    for &num in values {
        let mut bytes = vec![(num & MASK) as u8];
        let mut n = num >> 7;

        while n > 0 {
            bytes.push((n & MASK | ODD_BIT) as u8);
            n >>= 7;
        }

        bytes.reverse();
        result.extend(bytes);
    }


    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = vec![];
    let mut buffer: u32 = 0;
    let mut is_incomplete = false;

    for &byte in bytes {
        buffer |= (byte & MASK as u8) as u32;
        is_incomplete = byte & (ODD_BIT as u8) != 0;

        if is_incomplete {
            buffer <<= 7
        } else {
            result.push(buffer);
            buffer = 0;
        }
    }

    if is_incomplete {
        return Err(Error::IncompleteNumber)
    }


    Ok(result)
}

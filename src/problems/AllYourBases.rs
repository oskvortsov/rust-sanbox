// https://exercism.org/tracks/rust/exercises/all-your-base

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    validate_bases(from_base, to_base)?; // оператора раннего возврата ошибки

    // Handle empty slice case
    if number.is_empty() {
        return Ok(vec![0]);
    }

    let decimal = to_decimal(number, from_base)?;
    Result::Ok(from_decimal(decimal, to_base))
}



fn validate_bases(from_base: u32, to_base: u32) -> Result<(), Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    Ok(())
}

fn to_decimal(digits: &[u32], base: u32) -> Result<u128, Error> {
    let mut result: u128 = 0;

    for &digit in digits {
        if digit >= base {
            return Err(Error::InvalidDigit(digit));
        }

        // Формула такая
        // 42
        // 1) 0 * 10 + 4
        // 2) 4 * 10 + 2
        // итого: 42
        // Check for potential overflow before multiplication
        if let Some(new_result) = result.checked_mul(base as u128)
            .and_then(|r| r.checked_add(digit as u128)) {
            result = new_result;
        } else {
            return Err(Error::InvalidDigit(digit)); // Using InvalidDigit for overflow
        }
    }

    Ok(result)
}

fn from_decimal(mut number: u128, base: u32) -> Vec<u32> {
    if number == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while number > 0 {
        digits.push((number % base as u128) as u32);
        number /= base as u128;
    }
    digits.reverse();
    digits
}


pub fn convert_first(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Result::Err(Error::InvalidInputBase)
    }
    if to_base <= 1 {
        return Result::Err(Error::InvalidOutputBase)
    }
    let mut buffer: u32 = 0;
    // parse to decimal
    for (index, value) in number.iter().enumerate() {
        if *value >= from_base {
            return Result::Err(Error::InvalidDigit(*value));
        }
        let pow = (number.len() - index - 1) as u32;
        buffer += *value * from_base.pow(pow);
    }
    let mut result = vec![];
    while buffer >= to_base {
        result.push(buffer % to_base);
        buffer /= to_base;
    }
    result.push(buffer);
    result.reverse();
    Result::Ok(result)
}
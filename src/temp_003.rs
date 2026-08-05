#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvertError {
    Overflow,
}

fn decimal_width(value: u8) -> u32 {
    match value {
        0..=9 => 1,
        10..=99 => 2,
        100..=255 => 3,
    }
}

fn convert_array_to_number(data: &[u8]) -> Result<u128, ConvertError> {
    data.iter().try_fold(0u128, |acc, &value| {
        let width = decimal_width(value);
        let factor = 10u128
            .checked_pow(width)
            .ok_or(ConvertError::Overflow)?;

        acc.checked_mul(factor)
            .and_then(|n| n.checked_add(value as u128))
            .ok_or(ConvertError::Overflow)
    })
}

fn main() {
    let first = [1, 2, 3, 4, 5];
    let second = [10, 20, 130, 40, 255];

    match convert_array_to_number(&first) {
        Ok(number) => println!("first  => {}", number),
        Err(err) => println!("first  => error: {:?}", err),
    }

    match convert_array_to_number(&second) {
        Ok(number) => println!("second => {}", number),
        Err(err) => println!("second => error: {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_single_digits() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(convert_array_to_number(&data), Ok(12345));
    }

    #[test]
    fn converts_multi_digit_values() {
        let data = [10, 20, 130, 40, 255];
        assert_eq!(convert_array_to_number(&data), Ok(102013040255));
    }

    #[test]
    fn handles_empty_input() {
        let data: [u8; 0] = [];
        assert_eq!(convert_array_to_number(&data), Ok(0));
    }

    #[test]
    fn handles_zero_values() {
        let data = [0, 0, 7];
        assert_eq!(convert_array_to_number(&data), Ok(7));
    }

    #[test]
    fn detects_overflow() {
        let data = [255; 20];
        assert_eq!(convert_array_to_number(&data), Err(ConvertError::Overflow));
    }
}

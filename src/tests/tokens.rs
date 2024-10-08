

#[cfg(test)]
mod tests {
    use crate::utils::structs::tokens::DigitType;

    #[test]
    fn digit_types_ordering() {
        assert_eq!(DigitType::Binary   >> DigitType::Binary,   true);
        assert_eq!(DigitType::Binary   >> DigitType::Octal,    true);
        assert_eq!(DigitType::Binary   >> DigitType::Digital,  true);
        assert_eq!(DigitType::Binary   >> DigitType::HexPoint, true);
        assert_eq!(DigitType::Binary   >> DigitType::Hex,      true);
        assert_eq!(DigitType::Binary   >> DigitType::Point,    true);

        assert_eq!(DigitType::Octal    >> DigitType::Binary,   false);
        assert_eq!(DigitType::Octal    >> DigitType::Octal,    true);
        assert_eq!(DigitType::Octal    >> DigitType::Digital,  true);
        assert_eq!(DigitType::Octal    >> DigitType::HexPoint, true);
        assert_eq!(DigitType::Octal    >> DigitType::Hex,      true);
        assert_eq!(DigitType::Octal    >> DigitType::Point,    true);

        assert_eq!(DigitType::Digital  >> DigitType::Binary,   false);
        assert_eq!(DigitType::Digital  >> DigitType::Octal,    false);
        assert_eq!(DigitType::Digital  >> DigitType::Digital,  true);
        assert_eq!(DigitType::Digital  >> DigitType::HexPoint, true);
        assert_eq!(DigitType::Digital  >> DigitType::Hex,      true);
        assert_eq!(DigitType::Digital  >> DigitType::Point,    true);

        assert_eq!(DigitType::HexPoint >> DigitType::Binary,   false);
        assert_eq!(DigitType::HexPoint >> DigitType::Octal,    false);
        assert_eq!(DigitType::HexPoint >> DigitType::Digital,  false);
        assert_eq!(DigitType::HexPoint >> DigitType::HexPoint, true);
        assert_eq!(DigitType::HexPoint >> DigitType::Hex,      true);
        assert_eq!(DigitType::HexPoint >> DigitType::Point,    true);

        assert_eq!(DigitType::Hex      >> DigitType::Binary,   false);
        assert_eq!(DigitType::Hex      >> DigitType::Octal,    false);
        assert_eq!(DigitType::Hex      >> DigitType::Digital,  false);
        assert_eq!(DigitType::Hex      >> DigitType::HexPoint, false);
        assert_eq!(DigitType::Hex      >> DigitType::Hex,      true);
        assert_eq!(DigitType::Hex      >> DigitType::Point,    false);

        assert_eq!(DigitType::Point    >> DigitType::Binary,   false);
        assert_eq!(DigitType::Point    >> DigitType::Octal,    false);
        assert_eq!(DigitType::Point    >> DigitType::Digital,  false);
        assert_eq!(DigitType::Point    >> DigitType::HexPoint, false);
        assert_eq!(DigitType::Point    >> DigitType::Hex,      false);
        assert_eq!(DigitType::Point    >> DigitType::Point,    true);
    }
}
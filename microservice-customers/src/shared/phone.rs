#[derive(Debug, PartialEq)]
pub enum PhoneValidationError {
    InvalidLength(usize),
    SpecialCharacters,
}

impl std::fmt::Display for PhoneValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoneValidationError::InvalidLength(len) => 
                write!(f, "Phone number must be at most 20 characters (got {})", len),
            PhoneValidationError::SpecialCharacters => 
                write!(f, "Phone number cannot contain special characters"),
        }
    }
}

pub fn validate_phone(phone: &str) -> Result<(), PhoneValidationError> {
    // Verifica o tamanho
    if phone.len() > 20 {
        return Err(PhoneValidationError::InvalidLength(phone.len()));
    }

    // Não pode conter caracteres especiais exceto + - () e espaço
    let allowed = |c: char| c.is_ascii_digit() || matches!(c, '+' | '-' | '(' | ')' | ' ');
    if !phone.chars().all(allowed) {
        return Err(PhoneValidationError::SpecialCharacters);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone() {
        assert_eq!(validate_phone("+55 11 91234-5678"), Ok(()));
        assert_eq!(validate_phone("(11)91234-5678"), Ok(()));
        assert_eq!(validate_phone("11912345678"), Ok(()));
    }

    #[test]
    fn test_invalid_length() {
        let long_phone = format!("+55 {}", "1".repeat(20));
        assert_eq!(validate_phone(&long_phone), Err(PhoneValidationError::InvalidLength(long_phone.len())));
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(validate_phone("11@91234-5678"), Err(PhoneValidationError::SpecialCharacters));
        assert_eq!(validate_phone("11#91234-5678"), Err(PhoneValidationError::SpecialCharacters));
        assert_eq!(validate_phone("11$91234-5678"), Err(PhoneValidationError::SpecialCharacters));
    }
}

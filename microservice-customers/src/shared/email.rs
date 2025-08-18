




#[derive(Debug, PartialEq)]
pub enum EmailValidationError {
    InvalidLength(usize),
    MissingAtSymbol,
    SpecialCharacters,
}

impl std::fmt::Display for EmailValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailValidationError::InvalidLength(len) => 
                write!(f, "Email must be at most 100 characters (got {})", len),
            EmailValidationError::MissingAtSymbol => 
                write!(f, "Email must contain the '@' character"),
            EmailValidationError::SpecialCharacters => 
                write!(f, "Email cannot contain special characters"),
        }
    }
}


pub fn validate_email(email: &str) -> Result<(), EmailValidationError> {
    // Verifica o tamanho
    if email.len() > 100 {
        return Err(EmailValidationError::InvalidLength(email.len()));
    }

    // Verifica se contém '@'
    if !email.contains('@') {
        return Err(EmailValidationError::MissingAtSymbol);
    }

    // Não pode conter caracteres especiais exceto . _ - @
    let allowed = |c: char| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '@');
    if !email.chars().all(allowed) {
        return Err(EmailValidationError::SpecialCharacters);
    }

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert_eq!(validate_email("usuario@email.com"), Ok(()));
        assert_eq!(validate_email("nome.sobrenome-123@email.com"), Ok(()));
        assert_eq!(validate_email("a@b.co"), Ok(()));
    }

    #[test]
    fn test_invalid_length() {
        let long_email = format!("{}@email.com", "a".repeat(95));
        assert_eq!(validate_email(&long_email), Err(EmailValidationError::InvalidLength(long_email.len())));
    }

    #[test]
    fn test_missing_at_symbol() {
        assert_eq!(validate_email("usuarioemail.com"), Err(EmailValidationError::MissingAtSymbol));
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(validate_email("user!@email.com"), Err(EmailValidationError::SpecialCharacters));
        assert_eq!(validate_email("user#email@email.com"), Err(EmailValidationError::SpecialCharacters));
        assert_eq!(validate_email("user$email@email.com"), Err(EmailValidationError::SpecialCharacters));
    }
}
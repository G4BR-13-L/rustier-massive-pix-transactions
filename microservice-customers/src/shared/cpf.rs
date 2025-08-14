


#[derive(Debug, PartialEq)]
pub enum CpfValidationError {
    InvalidLength(usize),
    NonDigitCharacters,
    AllDigitsEqual,
    InvalidCheckDigits,
}

impl std::fmt::Display for CpfValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpfValidationError::InvalidLength(len) => 
                write!(f, "CPF deve ter 11 dígitos (recebido {})", len),
            CpfValidationError::NonDigitCharacters => 
                write!(f, "CPF deve conter apenas dígitos"),
            CpfValidationError::AllDigitsEqual => 
                write!(f, "CPF não pode ter todos dígitos iguais"),
            CpfValidationError::InvalidCheckDigits => 
                write!(f, "Dígitos verificadores inválidos"),
        }
    }
}

pub fn validate_cpf(cpf: &str) -> Result<(), CpfValidationError> {
    let digits: Vec<char> = cpf.chars().filter(|c| c.is_digit(10)).collect();
    
    // Verifica se contém apenas dígitos
    if digits.len() != cpf.chars().filter(|c| c.is_digit(10)).count() {
        return Err(CpfValidationError::NonDigitCharacters);
    }
    
    // Verifica o tamanho
    if digits.len() != 11 {
        return Err(CpfValidationError::InvalidLength(digits.len()));
    }
    
    // Verifica se todos os dígitos são iguais
    if digits.iter().all(|&c| c == digits[0]) {
        return Err(CpfValidationError::AllDigitsEqual);
    }
    
    // Calcula o primeiro dígito verificador
    let first_verifier = calculate_verifier(&digits, 9);
    if digits[9].to_digit(10) != Some(first_verifier) {
        return Err(CpfValidationError::InvalidCheckDigits);
    }
    
    // Calcula o segundo dígito verificador
    let second_verifier = calculate_verifier(&digits, 10);
    if digits[10].to_digit(10) != Some(second_verifier) {
        return Err(CpfValidationError::InvalidCheckDigits);
    }
    
    Ok(())
}

fn calculate_verifier(digits: &[char], length: usize) -> u32 {
    let sum = (0..length)
        .map(|i| digits[i].to_digit(10).unwrap() * ((length + 1) - i) as u32)
        .sum::<u32>();
    
    let verifier = (sum * 10) % 11;
    if verifier == 10 { 0 } else { verifier }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_cpf() {
        assert_eq!(validate_cpf("529.982.247-25"), Ok(()));
        assert_eq!(validate_cpf("52998224725"), Ok(()));
        assert_eq!(validate_cpf("111.444.777-35"), Ok(()));
    }
    
    #[test]
    fn test_non_digit_characters() {
        assert_eq!(
            validate_cpf("ABC.DEF.GHI-JK"),
            Err(CpfValidationError::NonDigitCharacters)
        );
        assert_eq!(
            validate_cpf("529.9A2.247-25"),
            Err(CpfValidationError::NonDigitCharacters)
        );
    }
    
    #[test]
    fn test_invalid_length() {
        assert_eq!(
            validate_cpf("123"),
            Err(CpfValidationError::InvalidLength(3))
        );
        assert_eq!(
            validate_cpf("123456789012"),
            Err(CpfValidationError::InvalidLength(12))
        );
    }
    
    #[test]
    fn test_all_digits_equal() {
        assert_eq!(
            validate_cpf("000.000.000-00"),
            Err(CpfValidationError::AllDigitsEqual)
        );
        assert_eq!(
            validate_cpf("11111111111"),
            Err(CpfValidationError::AllDigitsEqual)
        );
    }
    
    #[test]
    fn test_invalid_check_digits() {
        assert_eq!(
            validate_cpf("123.456.789-00"),
            Err(CpfValidationError::InvalidCheckDigits)
        );
        assert_eq!(
            validate_cpf("529.982.247-26"),
            Err(CpfValidationError::InvalidCheckDigits)
        );
    }
    
    #[test]
    fn test_empty_string() {
        assert_eq!(
            validate_cpf(""),
            Err(CpfValidationError::InvalidLength(0))
        );
    }
}
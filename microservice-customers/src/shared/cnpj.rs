#[derive(Debug, PartialEq)]
pub enum CnpjValidationError {
    InvalidLength(usize),
    NonDigitCharacters,
    AllDigitsEqual,
    InvalidCheckDigits,
}

impl std::fmt::Display for CnpjValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CnpjValidationError::InvalidLength(len) =>
                write!(f, "CNPJ deve ter 14 dígitos (recebido {})", len),
            CnpjValidationError::NonDigitCharacters =>
                write!(f, "CNPJ deve conter apenas dígitos"),
            CnpjValidationError::AllDigitsEqual =>
                write!(f, "CNPJ não pode ter todos dígitos iguais"),
            CnpjValidationError::InvalidCheckDigits =>
                write!(f, "Dígitos verificadores inválidos"),
        }
    }
}

pub fn validate_cnpj(cnpj: &str) -> Result<(), CnpjValidationError> {
    let digits: Vec<char> = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();

    // Verifica se há caracteres não numéricos
    if digits.len() != cnpj.chars().count() {
        return Err(CnpjValidationError::NonDigitCharacters);
    }

    // Verifica o tamanho
    if digits.len() != 14 {
        return Err(CnpjValidationError::InvalidLength(digits.len()));
    }

    // Verifica se todos os dígitos são iguais
    if digits.iter().all(|&c| c == digits[0]) {
        return Err(CnpjValidationError::AllDigitsEqual);
    }

    // Converte para vetor de inteiros
    let nums: Vec<u32> = digits.iter().map(|c| c.to_digit(10).unwrap()).collect();

    // Calcula verificadores
    let first_verifier = calculate_verifier(&nums, 12);
    let second_verifier = calculate_verifier(&nums, 13);

    if nums[12] != first_verifier || nums[13] != second_verifier {
        return Err(CnpjValidationError::InvalidCheckDigits);
    }

    Ok(())
}

fn calculate_verifier(digits: &[u32], length: usize) -> u32 {
    let weights = if length == 12 {
        vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]
    } else {
        vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]
    };

    let sum: u32 = digits[..length].iter().zip(weights.iter()).map(|(d, w)| d * w).sum();
    let resto = sum % 11;

    if resto < 2 { 0 } else { 11 - resto }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cnpjs() {
        assert_eq!(validate_cnpj("04.252.011/0001-10"), Ok(()));
        assert_eq!(validate_cnpj("04252011000110"), Ok(()));
        assert_eq!(validate_cnpj("40.688.134/0001-61"), Ok(()));
        assert_eq!(validate_cnpj("40688134000161"), Ok(()));
    }

    #[test]
    fn test_non_digit_characters() {
        assert_eq!(
            validate_cnpj("04.252.011/0001-AA"),
            Err(CnpjValidationError::NonDigitCharacters)
        );
    }

    #[test]
    fn test_invalid_length() {
        assert_eq!(
            validate_cnpj("123"),
            Err(CnpjValidationError::InvalidLength(3))
        );
        assert_eq!(
            validate_cnpj("123456789012"),
            Err(CnpjValidationError::InvalidLength(12))
        );
    }

    #[test]
    fn test_all_digits_equal() {
        assert_eq!(
            validate_cnpj("00000000000000"),
            Err(CnpjValidationError::AllDigitsEqual)
        );
    }

    #[test]
    fn test_invalid_check_digits() {
        assert_eq!(
            validate_cnpj("04.252.011/0001-00"),
            Err(CnpjValidationError::InvalidCheckDigits)
        );
    }
}

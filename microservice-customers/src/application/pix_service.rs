use deadpool_postgres::Client;
use uuid::Uuid;

use crate::{
    application::{account_service, customer_service, dto::pix_key_dto::CreatePixKeyRequest},
    domain::{
        account::{self, Account},
        customer::{self, Customer},
        enums::AccountType,
        pix_key::{PixKey, PixKeyType},
    },
    infraestructure::{
        db::pix_key_repo,
        error::{ApiError, MyError},
    },
    shared::{
        cnpj::{self, validate_cnpj, CnpjValidationError},
        cpf, email, phone,
    },
};

pub async fn create_pix_key(
    client: &mut Client,
    pix_key_info: &mut CreatePixKeyRequest,
    customer_id: Uuid,
) -> Result<PixKey, MyError> {
    let uiuiuid = customer_id.clone();
    let customer: Customer = customer_service::get_customer_by_id(client, customer_id).await?;
    let account: Account = account_service::get_account_by_customer_id(client, customer_id).await?;

    if pix_key_info.key_type == PixKeyType::CNPJ {
        let cnpj_validation_result = cnpj::validate_cnpj(&pix_key_info.key_value);

        match cnpj_validation_result {
            Ok(()) => println!("CNPJ validated."),
            Err(error) => {
                return Err(MyError::ApiError(ApiError {
                    msg: error.to_string(),
                }))
            }
        }
    }

    if pix_key_info.key_type == PixKeyType::CPF {
        let cpf_validation_result = cpf::validate_cpf(&pix_key_info.key_value);

        match cpf_validation_result {
            Ok(()) => println!("CPF validated."),
            Err(error) => {
                return Err(MyError::ApiError(ApiError {
                    msg: error.to_string(),
                }))
            }
        }

        if customer.cpf != pix_key_info.key_value {
            return Err(MyError::ApiError(ApiError {
                msg: "CPF does not match the customer's CPF".to_string(),
            }));
        }
    }

    if pix_key_info.key_type == PixKeyType::EMAIL {
        let email_validation_result = email::validate_email(&pix_key_info.key_value);

        match email_validation_result {
            Ok(()) => println!("Email validated."),
            Err(error) => {
                return Err(MyError::ApiError(ApiError {
                    msg: error.to_string(),
                }))
            }
        }

        if customer.email != pix_key_info.key_value {
            return Err(MyError::ApiError(ApiError {
                msg: "Email does not match the customer's email".to_string(),
            }));
        }
    }

    if pix_key_info.key_type == PixKeyType::PHONE {
        if let Some(phone) = &customer.phone {
            let phone_validation_result = phone::validate_phone(&pix_key_info.key_value);

            match phone_validation_result {
                Ok(()) => println!("Phone validated."),
                Err(error) => {
                    return Err(MyError::ApiError(ApiError {
                        msg: error.to_string(),
                    }))
                }
            }

            if phone.as_str() != pix_key_info.key_value {
                return Err(MyError::ApiError(ApiError {
                    msg: "Phone number does not match the customer's phone number".to_string(),
                }));
            }
        } else {
            return Err(MyError::ApiError(ApiError {
                msg: "Customer does not have a phone number".to_string(),
            }));
        }
    }

    if pix_key_info.key_type == PixKeyType::RANDOM {
        pix_key_info.key_value = uuid::Uuid::new_v4().to_string();
    }

    let new_pix_key = pix_key_repo::create_pix_key(client, pix_key_info, account.id).await?;

    Ok(new_pix_key)
}

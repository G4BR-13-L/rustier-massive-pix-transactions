CREATE TYPE pix_key_type_new AS ENUM ('CPF', 'CNPJ', 'EMAIL', 'PHONE', 'RANDOM');

ALTER TABLE pix_keys
    ALTER COLUMN key_type TYPE pix_key_type_new
    USING key_type::text::pix_key_type_new;

DROP TYPE pix_key_type;
ALTER TYPE pix_key_type_new RENAME TO pix_key_type;
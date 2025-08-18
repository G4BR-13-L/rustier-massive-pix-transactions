INSERT INTO public.pix_keys
(key_value, key_type, account_id)
VALUES($1, $2, $3)
RETURNING $table_fields;
INSERT INTO public.accounts
(customer_id, account_type, currency)
VALUES($1, $2, $3)
RETURNING $table_fields;
INSERT INTO public.customers
(full_name, email, cpf)
VALUES ($1, $2, $3)
RETURNING $table_fields;

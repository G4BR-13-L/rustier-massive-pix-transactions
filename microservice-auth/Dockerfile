# =========================
# Etapa 0 — Base Chef
# =========================
FROM messense/rust-musl-cross:x86_64-musl AS chef

WORKDIR /app

# Para builds offline de sqlx
ENV SQLX_OFFLINE=true
# ENV DATABASE_URL=postgres://postgres:postgres@localhost:5451/customers
# ENV JWT_SECRET=uma_chave_super_secreta_que_ninguem_sabe


# Instala cargo-chef
RUN cargo install cargo-chef

# =========================
# Etapa 1 — Planner
# =========================
FROM chef AS planner

WORKDIR /app

# Copia apenas Cargo.toml e Cargo.lock para gerar recipe
COPY Cargo.toml Cargo.lock ./

# Cria src vazio para não invalidar cache
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Gera recipe.json para cache de dependências
RUN cargo chef prepare --recipe-path recipe.json

# =========================
# Etapa 2 — Builder
# =========================
FROM chef AS builder

WORKDIR /app

# Copia recipe.json do planner
COPY --from=planner /app/recipe.json recipe.json

# Compila todas as dependências (cacheável)
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Copia código real
COPY . .

# Compila apenas o binário final
RUN cargo build --release --target x86_64-unknown-linux-musl --bin microservice-auth

# =========================
# Etapa 3 — Runtime minimal
# =========================
FROM scratch AS runtime

WORKDIR /app

# Copia o binário estático final
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/microservice-auth .

EXPOSE 8081
ENV RUST_LOG=info

ENTRYPOINT ["/app/microservice-auth"]


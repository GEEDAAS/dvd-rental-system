# ---- Etapa 1: Compilación (Builder) ----
# Usamos la imagen oficial de Rust como base para compilar nuestro código.
FROM rust:1.90 as builder

# Establecemos el directorio de trabajo dentro del contenedor.
WORKDIR /usr/src/app

# Copiamos nuestro código fuente al contenedor.
COPY . .

# Compilamos el proyecto en modo 'release' para máxima optimización.
# Esto genera un único archivo binario ejecutable.
RUN cargo build --release

# ---- Etapa 2: Ejecución (Final) ----
# Usamos una imagen base de Debian muy ligera. No necesitamos
# todas las herramientas de Rust para solo ejecutar el binario.
FROM debian:bookworm-slim

# Copiamos SOLAMENTE el binario compilado desde la etapa 'builder'.
# Esto mantiene la imagen final pequeña y reduce la superficie de ataque.
COPY --from=builder /usr/src/app/target/release/api /usr/local/bin/api

# Exponemos el puerto 8080, que es el que usa nuestra aplicación.
EXPOSE 8080

# El comando que se ejecutará cuando el contenedor se inicie.
CMD ["api"]
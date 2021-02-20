FROM ekidd/rust-musl-builder:stable AS cargo-build

ARG BINARY_NAME
# as binary name but - -> _ 
ARG DEP_NAME

COPY Cargo.toml Cargo.toml

RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/${DEP_NAME}*

COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl


FROM alpine:3.10

ARG BINARY_NAME
LABEL authors="red.avtovo@gmail.com"

COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/${BINARY_NAME} /opt/

ENV RUST_LOG="info"
RUN apk add --no-cache ca-certificates && update-ca-certificates
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

CMD ["/opt/${BINARY_NAME}"]
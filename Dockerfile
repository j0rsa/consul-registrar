FROM alpine:3.10
LABEL authors="red.avtovo@gmail.com"
COPY ./target/release/consul-registrar /opt/
CMD ["/opt/consul-registrar"]
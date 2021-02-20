FROM alpine:3.10
LABEL authors="red.avtovo@gmail.com"
ARG TARGET=.
COPY ./target/$TARGET/release/consul-registrar /opt/
CMD ["/opt/consul-registrar"]
FROM rust:1.54

WORKDIR /usr/src/ls-mira-services
COPY . .

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/ls-mira-services"]

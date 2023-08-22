FROM rust:1-bookworm

WORKDIR /usr/src/krabby

COPY . .

RUN cargo install --path .

CMD ["krabby"]

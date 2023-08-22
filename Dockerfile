FROM rust:1-alpine

WORKDIR /usr/src/krabby

COPY . .

RUN cargo install --path .

CMD ["krabby"]

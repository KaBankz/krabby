FROM rust:1-slim

WORKDIR /usr/src/krabby

COPY . .

RUN cargo install --path .

CMD ["krabby"]

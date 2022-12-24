FROM rust:1.66.0

WORKDIR /usr/src/krabby

COPY . .

RUN cargo install --path .

CMD ["krabby"]

FROM rust:1.91.1-alpine3.20 as builder
WORKDIR /reddit-clone

COPY . .

RUN cargo install --path .

CMD ["ls"]



FROM rust:1.64
WORKDIR /usr/src/rust-book-web-server
COPY . .
RUN cargo install --path .
CMD ["rust-book-web-server"]
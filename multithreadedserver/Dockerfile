FROM rust:1.64
WORKDIR /usr/src/single-threaded-rust-book-web-server
COPY . .
RUN cargo install --path .
ENV PORT=8000
CMD ["single-threaded-rust-book-web-server"]
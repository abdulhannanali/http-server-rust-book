FROM rust:1.64
WORKDIR /usr/src/multithreadedserver
COPY . .
RUN cargo install --path .
ENV PORT=8000
CMD ["multithreadedserver"]
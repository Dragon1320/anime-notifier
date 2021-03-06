FROM rust:slim-buster AS builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

# create a dummy project to make use of dockers build caching
# if cargo/lock files dont change, built deps will be reused
RUN cargo init
COPY Cargo.lock Cargo.toml ./
RUN cargo build --target=x86_64-unknown-linux-musl --release --locked

# remove default source files generated by cargo init and dummy build
RUN rm src/*.rs
RUN rm target/x86_64-unknown-linux-musl/release/deps/anime_notifier*

# build the project
COPY src ./src
RUN cargo build --target=x86_64-unknown-linux-musl --release --locked

# final container
FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/anime-notifier ./

# non-root user
USER 1000

EXPOSE 3000

ENTRYPOINT ["./anime-notifier"]

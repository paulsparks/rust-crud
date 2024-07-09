FROM rust:1-bullseye

# Install additional software on dev container
RUN cargo install diesel_cli --no-default-features --features postgres && \
  rustup component add rustfmt
FROM rust:latest

# project workspace
WORKDIR /nano
# install aditional dependencies
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/* && rustup component add rustfmt
# cargo binaries
# RUN cargo install --root cargo-edit
# RUN cargo install --root cargo-watch
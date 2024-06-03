#LABEL authors="fabio"
# My current Rust version
FROM rust:1.76

# 2. Copying my directory into the docker image
COPY ./ ./

# Building the application with cargo
RUN cargo build --release

# Run the binary
CMD ["./target/release/truepkmn"]

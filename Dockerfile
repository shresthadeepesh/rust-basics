# Use a minimal base image with Rust and OpenSSL development packages installed
FROM rust:1.72.1

# Create a new directory to work in
WORKDIR /app

# Copy your Rust project files into the container
COPY . .

RUN cargo build --release

# Expose the port your HTTP server will listen on
EXPOSE 8080

# Command to run your HTTP server
CMD ["./target/release/basics"]

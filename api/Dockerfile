# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy only the necessary files for building the application
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=0 /app/target/release/server ./

# Expose the port the API will run on
EXPOSE 8080

# Set the command to run the application
CMD ["./server"]
# Use the official Rust image as a base
FROM messense/rust-musl-cross:x86_64-musl as builder

# Environments
ENV PORT=3070

# Set the working directory
WORKDIR /usr/src/service_docurust

# Copy the application code
COPY . .

# Make the setup script executable and run it
RUN chmod +x setup.sh && ./setup.sh

# Set the entry point for the container
ENTRYPOINT ["/usr/src/service_docurust/target/x86_64-unknown-linux-musl/release/service_docurust"]

# Expose the port the server is running on
EXPOSE 3070

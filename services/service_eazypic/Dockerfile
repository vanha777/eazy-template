# Use the official Rust image as a base
FROM messense/rust-musl-cross:x86_64-musl as builder

# Environments
ENV PORT=1010
ENV PHOTOSHOP_CLIENT_ID=a44eef1666f348a49c4a620026e90742
ENV PHOTOSHOP_CLIENT_SECRET=p8e-q1YlUvJoEaoOlBTaxq6W3cjDK6Zs0pvw
ENV OPENAI_API_KEY=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
ENV HEYGEN_API_KEY=NGM1NjQ3ZjU2ZTJiNDc1YmJmZDI2NDgxMDRiNTQ1NTQtMTY5NjEzNzQ2NQ==
ENV AWS_ACCESS_KEY_ID=AKIA6COXGD6KESU77HL3
ENV AWS_SECRET_ACCESS_KEY=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
ENV AWS_REGION=ap-southeast-4
ENV AWS_BUCKET=eazypic
ENV AWS_IMAGE_KEY=images/

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

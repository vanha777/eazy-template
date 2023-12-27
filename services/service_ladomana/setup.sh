#!/bin/bash
# setup.sh

# Ensure non-interactive installation
export DEBIAN_FRONTEND=noninteractive

# Update package list
apt-get update

# Install wkhtmltopdf
apt-get install -y wkhtmltopdf

# Install pkg-config
apt-get install -y pkg-config

# Install OpenSSL development libraries
apt-get install -y libssl-dev

# Install Apache HTTP Server (equivalent to httpd in Yum)
apt-get install -y apache2

# Install CA certificates
apt-get install -y ca-certificates

# Clean up the package cache to reduce image size
apt-get clean && rm -rf /var/lib/apt/lists/*

# Compile the Rust application
cargo build --release --target x86_64-unknown-linux-musl -p service_eazypic
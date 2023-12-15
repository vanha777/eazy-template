#!/bin/bash
# setup.sh

# Ensure non-interactive installation
export DEBIAN_FRONTEND=noninteractive

# Update package list
apt-get update

# # Install wkhtmltopdf
# apt-get install -y wkhtmltopdf

# Install OpenSSL (the package name might just be 'openssl' in Debian/Ubuntu)
apt-get install -y openssl

# Install OpenSSL development libraries
apt-get install -y libssl-dev

# Install pkg-config
apt-get install -y pkg-config

# Install Apache HTTP Server (equivalent to httpd in Yum)
apt-get install -y apache2

# Clean up the package cache to reduce image size
apt-get clean && rm -rf /var/lib/apt/lists/*

# Compile the specific Rust service
cargo build --release -p service_eazypic

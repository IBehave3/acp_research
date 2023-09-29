#!/bin/bash

# Paths to the certificate and CA bundle files
certificate_file="./ssl/certificate.crt"
ca_bundle_file="./ssl/ca_bundle.crt"
ssl_bundle_file="./ssl/ssl_bundle.crt"

# Check if the certificate file exists
if [ ! -f "$certificate_file" ]; then
  echo "Certificate file '$certificate_file' not found."
  exit 1
fi

# Check if the CA bundle file exists
if [ ! -f "$ca_bundle_file" ]; then
  echo "CA bundle file '$ca_bundle_file' not found."
  exit 1
fi

# Check if the CA bundle already file exists
if [ -f "$ssl_bundle_file" ]; then
  echo "SSL bundle file '$ssl_bundle_file' already exists. pls delete only if you really want to replace it!"
  exit 1
fi

# Combine certificate and CA bundle into a single file
cat "$certificate_file" "$ca_bundle_file" > $ssl_bundle_file

echo "Certificate and CA bundle combined into 'ssl_bundle.crt'."

#!/bin/bash

if [[ $# -lt 2 ]]; then
    echo "Usage: bash $0 <input.key> <output>"
    exit
fi;

openssl asn1parse -genconf $1 -out newkey.der
openssl rsa -in newkey.der -inform der -out $2
rm newkey.der
openssl rsa -in $2 -check

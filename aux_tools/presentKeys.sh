#!/bin/bash

echo "Public key:"
cat 8gwifikeys/8gwifi.pub
sleep 2
echo "Private key:"
cat 8gwifikeys/8gwifiOFFICIAL.key
read
echo "Running extract.py on public key to find e"
sleep 1
python extract.py 8gwifikeys/8gwifi.pub
read
echo "Printing sections of private key"
sleep 1
openssl rsa -in 8gwifikeys/8gwifiOFFICIAL.key -text
read
echo "Extracting p and q from private key"
sleep 1
for i in {1..2}; do openssl rsa -in 8gwifikeys/8gwifiOFFICIAL.key -text | grep prime$i -A5 | tr -d " " | tr -d "\n" | tr -d ":" | cut -c 7- | sed 's/^/0x/'; done
read
echo "Printing p and q and decimals"
sleep 1
python print.py
read
echo "Running keyTest with predefined p, q, and e..."
sleep 1
cargo run keyTest
read
echo "Writing keyTest output to file 'keyTestresults.key'"
cargo run keyTest > keyTestresults.key
sleep 1
echo "Recreating private key with results"
sleep 1
bash keyRegen.sh keyTestresults.key 8gwifikeys/8gwifi.key
echo "Running diff with official key and created key..."
diff 8gwifikeys/8gwifiOFFICIAL.key 8gwifikeys/8gwifi.key
sleep 2
echo "All good!"
sleep 5
echo "Side-by-side comparison:"
diff -y 8gwifikeys/8gwifiOFFICIAL.key 8gwifikeys/8gwifi.key
read
echo "Cleaning up"
rm keyTestresults.key 8gwifikeys/8gwifi.key

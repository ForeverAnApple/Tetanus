import sys
from Crypto.PublicKey import RSA

args=len(sys.argv)
if args < 2:
    print("Usage: python extract_e.py <public key>")
    exit(0)

key=RSA.importKey(open(sys.argv[1], 'r').read())
print("The E portion of your public key is: 0x" + format(key.e, 'x'))

import sys
from Crypto.PublicKey import RSA

key=RSA.importKey(open(sys.argv[1], 'r').read())
e=hex(key.e)
print("The E portion of your public key is: 0x" + format(key.e, 'x'))

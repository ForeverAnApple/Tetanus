import sys
from Crypto.PublicKey import RSA

args=len(sys.argv)
if args < 2:
    print("Usage: python extract_e.py <public key> <optional: GCD result from key>")
    exit(0)

key=RSA.importKey(open(sys.argv[1], 'r').read())
print("The E portion of your public key is: 0x" + format(key.e, 'x'))
print("The N portion of your public key is: 0x" + format(key.n, 'x'))

#if args >= 3:
#    print("\nFinding p/q...")
#    p=sys.argv[2]
#    print("With a p of " + format(long(p), 'x') + ", q will be: ")
#    q=0xa/int(sys.argv[2][2:],16)
#    print("0x" + q)
#    print("NOTE: when generating the key p and q may be switched, so there are two potential private keys at this point")

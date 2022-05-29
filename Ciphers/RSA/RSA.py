"""
Name: Dominic Wojewodka
Date: 11-18-2020
"""

from functools import reduce
"""
HELPER FUNCTIONS
"""

"""
mulexp
implements the recursive binary modular exponentiation algoritm for efficiency
input: b (message) e or d (public key or private key) m(modulus-eulers totient function)
output: integer (b^e(mod m))
"""
def mulexp(b,e,m):
    if (e == 0):
        return (((b**0)% m) % m)
    elif ((e % 2) == 0):
        return ((mulexp(b,(e/2),m)**2) % m)
    else:
        return ((mulexp(b, (e-1), m)*b) %m)

"""
StoA
AtoS
converts a string to a int representing the ascii values using ord() or vice versa
input: string 
output: int 
"""
def StoA (s):
    Alist = list(map((lambda x: str(x)),(map(ord, s))))
    return int(reduce(lambda x,y: x+y, Alist))

def AtoS (A):
    "this will be hard"
    pass
"""
ENCRYPT/DECRYPT
"""

"""
encrypt
uses modular arithmetic to convert the plaintext to ciphertext
takes (b^e)%m and returns that number as the plaintext
"""
def encrypt (Ptext,e,m):
    b = StoA(Ptext)
    return mulexp(b,e,m)
    
"""
decrypt
opposite of encrypt
"""
def decrypt(Ctext,d,m):
    b = Ctext
    return mulexp(b,d,m)

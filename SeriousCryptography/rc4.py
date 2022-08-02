#Key Scheduling Algorithm
def ksa(K):
    n = len(K)
    S = [x for x in range(256)]
    j = 0
    for i in range(256):
        j = (j + S[i] + K[i % n]) % 256
        #swap
        S[i], S[j] = S[j], S[i]
    return S

def encrypt(P):
    #Generate State
    S = ksa([0,0,0,0,0])
    #Generate Keystream
    i = 0
    j = 0
    KS = [0 for x in range(len(P))]
    for b in range(len(P)):
        i = (i+1) % 256
        j = (j + S[i]) % 256
        #swap
        S[i] , S[j] = S[j], S[i]
        KS[b] = S[(S[i] + S[j]) % 256]
    #XOR Plaintext with KeyStream
    C = []
    for i in range(len(P)):
        C.append(KS[i] ^ P[i])
    return C

if __name__ == "__main__":
    C = encrypt([0,1,0,1])
    print(C)

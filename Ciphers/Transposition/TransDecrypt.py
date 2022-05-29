#Program for decrypting a Transposition cipher given the key
#Dominic Wojewodka 
#March 2021
""""""
import math
#decrypt
def decrypt(cipherText,key):
    cols = math.ceil(len(cipherText)/key)
    lst = []
    #use the algorithm to decrypt
    #counter
    cnt = 0
    #positon in 1d array
    i = 0
    #x position in 2d array
    x = 0
    while(cnt<len(cipherText)):
        print(i)
        if (x == cols):
            i -= (len(cipherText)-cols)
            x = 0
        
        lst.append(cipherText[i])
        
        cnt += 1
        i += cols
        x += 1
    return lst
#main
cipherText = input("Input the ciphertext: " )
key = int(input("Enter the key: "))
print(decrypt(cipherText, key))

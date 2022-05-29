# Transposition Cipher Encrypt
#
#Creator: Dominic Wojewodka
#Date: 31, October 2020

#Helper Functions:
   #read:
    #takes a string L and returns a list of each element in the string

   #row:
    #Creates a list of lists representing the rows in the list

    #LtoString
    #converts Cipherlist into the ciphertext
def read(S):
    L = []
    if (S == ""):
        return L 
    else:
        L.append(S[0])
        return L + read(S[1:])

def row(L,n,k):
    if (n == 1):
        return [L]
    else:
        return [L[0:k]] + row(L[k:],(n-1),k) 

def LtoString(L):
    S = ""
    for i in L:
        S = S + i
    return S  
#Encrypts the list of strings defined earlier and returns
#a string of ciphertext
#
#Input: String, Int K (the key)
#Output: String (ciphertext)
#
def encrypt(S,k):
    L = read(S)
    #first figure out how many rows we need
    cnt = 0
    r = 0
    for i in L:
        if (cnt % k == 0):
            r += 1
        cnt += 1
   #Lists of Lists representing rows 
    LoL = row(L, r, k)
    
    #now use the algoritm to fill the cipherlist
    Cipherlist = []
    i =0
    j =0
    while (i < r)and (j < k):
        if ( i == (r-1) ) and ( j == (k-1) ):
            Cipherlist.append(LoL[i][j])
            i += 1
            #ends the loop
        elif (len(LoL[(r-1)]) <= j) and (i == (r-2)):
            Cipherlist.append(LoL[i][j])
            i = 0
            j += 1
        elif (len(LoL[(r-1)]) <= j) and (j == (k-1)) and (i == (r-2)):
            Cipherlist.append(LoL[i][j])
            i = r
            #ends the loop
        elif(i < (r-1)):
            Cipherlist.append(LoL[i][j])
            i += 1
        elif(i == (r-1)):
            Cipherlist.append(LoL[i][j])
            i = 0
            j += 1
   #converts the cipherlist into a string representing the ciphertext
    return LtoString(Cipherlist)
        
#for running the program 
plaintext = input("Enter your text to encrypt: ")
key = int(input("input your numberic key: "))
print(encrypt(plaintext,key))
    
    

plaintext = input("Enter Your Plaintext: ").replace(" ", "")
arrPlain = []
for i,c in enumerate(plaintext):
    arrPlain.append(c) 
    
#print(a)

#okay now this is where the fun begins 

PlainAlphabet = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"]
CipherAlphabet = ["","","","","","","","","","","","","","","","","","","","","","","","","",""]
shift = int(input("Ceaser Shift value [Default 3]: "))

if shift == "": 
    shift = 3

c1 = 0
c2 = 0
while c1 <=25:
    if c1<=25-shift:
        CipherAlphabet[c1] = PlainAlphabet[c1+shift]
        c1 +=1
    else: 
        CipherAlphabet[c1] = PlainAlphabet[c2]
        c1 +=1
        c2 +=1



arrCipher = arrPlain 
c3 =0
while c3 < len(arrCipher):
    arrCipher[c3] = CipherAlphabet[PlainAlphabet.index(arrPlain[c3])]
    c3 +=1


for i in arrCipher: 
    print(i, end="")

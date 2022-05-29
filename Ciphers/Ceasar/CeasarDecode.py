###
#Ceasar Cipher Decryption program, however a key is not required.
#Dominic Wojewodka
###


ciphertext = input("Input your ciphertext: ").replace(" ","")
arrCipher = [] 
for i,c in enumerate(ciphertext):
    arrCipher.append(c)
 
 

""" 
NOTES 

The goal is to decode a Ceasar shift cipher with an unknown shift value
What the code does: 
    
    1. Establish an alphabet of the most commonly used letters in the english language called plainAlphabet[]
    2. Counts the frequency of each letter used in the ciphertext, storing them in used[] and freq[] and combining these into
    the hashmap freqanal{} 
    3. Determines the shift value based on the most commonly used letter, which the code assumes to be e, for ease of frequency analysis 
    4. Creates a cipheralphabet based on the shift value 
    5. Uses the PlainPlain[] array and the cipherAlphabet[] array to convert the ciphertext into plaintext
    
"""
PlainPlain = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"] 
plainAlphabet = ["e","t","a","r","i","o","n","s","h","d","l","u","w","m","f","c","g","y","p","b","k","v","j","x","q","z"]
cipherAlphabet= ["","","","","","","","","","","","","","","","","","","","","","","","","",""]
used = []
freq =[] 

counter =0 
while counter < len(arrCipher):
    isUsed = False 
    for i,c in enumerate(used):
        if c == arrCipher[counter]:
            isUsed = True
            freq[i] += 1 
    if isUsed == False:
        used.append(arrCipher[counter])
        freq.append(1)
    counter +=1

#Turning used[] and freq[] and making a hashmap/dictionary
freqanal = {
        }
counter2 =0
while counter2 < len(used):
    freqanal[used[counter2]] = freq[counter2]
    counter2 +=1


#This code sorts the hashmap in decending order
sortFreqAnal = sorted(freqanal.items(), key=lambda x: x[1], reverse=True)


shiftval = 0

if sortFreqAnal[0][0]=="f":
    shiftval = 1
if sortFreqAnal[0][0] == "g":
    shiftval = 2
if sortFreqAnal[0][0] == "h":
    shiftval = 3
if sortFreqAnal[0][0] == "i":
    shiftval = 4
if sortFreqAnal[0][0] == "j": 
    shiftval = 5
if sortFreqAnal[0][0] == "k":
    shiftval= 6
if sortFreqAnal[0][0] == "l":
    shiftval = 7
if sortFreqAnal[0][0] == "m":
    shiftval = 8
if sortFreqAnal[0][0] == "n":
    shiftval = 9
if sortFreqAnal[0][0] == "o":
    shiftval = 10
if sortFreqAnal[0][0] == "p":
    shiftval = 11
if sortFreqAnal[0][0] == "q":
    shiftval = 12
if sortFreqAnal[0][0] == "r":
    shiftval = 13
if sortFreqAnal[0][0] == "s":
    shiftval = 14
if sortFreqAnal[0][0] == "t":
    shiftval = 15
if sortFreqAnal[0][0] == "u":
    shiftval = 16
if sortFreqAnal[0][0] == "v":
    shiftval = 17
if sortFreqAnal[0][0] == "w":
    shiftval = 18
if sortFreqAnal[0][0] == "x":
    shiftval = 19
if sortFreqAnal[0][0] == "y":
    shiftval = 20
if sortFreqAnal[0][0] == "z":
    shiftval = 21
if sortFreqAnal[0][0] == "a":
    shiftval = 22
if sortFreqAnal[0][0] == "b":
    shiftval = 23
if sortFreqAnal[0][0] == "c":
    shiftval = 24
if sortFreqAnal[0][0] == "d":
    shiftval = 25
#constructs the cipheralphabet using the shift value
x =0
y= 0
while x <= 25:
    if x <= 25-shiftval:
        cipherAlphabet[x] = PlainPlain[x+shiftval]
        x+=1
    else:
        cipherAlphabet[x] = PlainPlain[y]
        x +=1
        y +=1

#convert ciphertext to plaintext

arrPlain = arrCipher

z=0
while z<len(arrPlain):
    arrPlain[z]= PlainPlain[cipherAlphabet.index(arrCipher[z])]
    z +=1


for i in arrPlain:
    print(i,end="")

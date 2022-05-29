#Dominic Wojewodka
#03/08/2021


#isEnglish
#this function will take a string and determine if it is english
#a standard ratio is passed as a parameter for the percentage of english that are required to be marked as english
def isEnglish(text,dictionary,ratio):
    #first convert the string into an array
    text = text.split(" ")
    length = len(text)
    count = 0
    #checks the words against the dictionary, counting the number of english words
    for i in text:
        if i in dictionary:
            count += 1
    #ratio of english to non english words
    PERCENT_ENGLISH = count/length
    print("Percent English: ", PERCENT_ENGLISH)
    if (PERCENT_ENGLISH >= ratio):
        return True
    else:
        return False
    
#dictionary
#file which opens the dictionary and stores the contents in an array
def dictionary():
    dictionaryFile = open("../CheckLang/englishWords.txt")
    dictionaryFile = dictionaryFile.read().splitlines()
    
    words = []
    for i in dictionaryFile:
        words.append(i)
    return words



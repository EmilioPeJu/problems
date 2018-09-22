def hangman(word, letters):
    missed=0
    word_counter=set(word)
    total_elements=len(word)
    for letter in letters:
        if letter in word_counter:
            word_counter.remove(letter)
            if len(word_counter)==0:
                return True
        else:
            missed+=1
            if missed==6:
                return False
    return False

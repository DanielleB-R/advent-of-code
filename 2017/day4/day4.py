def is_valid(passphrase):
    words = passphrase.split()
    return len(words) == len(set(words))


def is_valid_anagram(passphrase):
    words = ["".join(sorted(w)) for w in passphrase.split()]
    return len(words) == len(set(words))


with open("input.dat") as infile:
    passphrases = [s.strip() for s in infile]

valid_passphrases = [p for p in passphrases if is_valid(p)]

print(len(valid_passphrases))

valid_anagram_passphrases = [p for p in passphrases if is_valid_anagram(p)]

print(len(valid_anagram_passphrases))

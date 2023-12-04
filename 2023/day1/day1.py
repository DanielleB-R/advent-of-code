import regex

def firstAndLastDigits(line):
    digits = getDigits(line)
    return digits[0] + digits[-1]

def sumFirstAndLast(lines):
    return sum(int(firstAndLastDigits(l)) for l in lines)

wordRe = regex.compile(r"(\d|one|two|three|four|five|six|seven|eight|nine)")

wordNumbers = {
    "one": "1",
    "1": "1",
    "two": "2",
    "2": "2",
    "three": "3",
    "3": "3",
    "four": "4",
    "4": "4",
    "five": "5",
    "5": "5",
    "six": "6",
    "6": "6",
    "seven": "7",
    "7": "7",
    "eight": "8",
    "8": "8",
    "nine": "9",
    "9": "9",
}

def getDigits(line):
    return [wordNumbers[m[0]] for m in wordRe.finditer(line, overlapped=True)]

with open("input.dat") as infile:
    print(sumFirstAndLast(infile))

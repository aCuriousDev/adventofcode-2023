import re

NUMBER_MAP = {
    'zero': 0,
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9
}


def get_int(line: str) -> int:
    p = re.compile(r'(?=([0-9]|' + '|'.join(NUMBER_MAP) + '))')

    first_int = p.findall(line)[0]
    second_int = p.findall(line)[-1]

    words = []
    for word in [first_int, second_int]:
        if word in NUMBER_MAP:
            words.append(str(NUMBER_MAP[word]))
        else:
            words.append(word)
    return int(''.join(words))


with open('./input', 'r') as f:
    total_sum = sum(get_int(line) for line in f.readlines())

print(f"Total sum of calibration values: {total_sum}")

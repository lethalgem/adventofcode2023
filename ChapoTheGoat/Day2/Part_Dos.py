import re
dad = 0
colors = ["blue", "red", "green"]


def find_max_color(line, color, max_num, matches):
    for match in matches:
        start_index = match.start()
        substring = line[:start_index][::-1]
        digit_match = re.search(r'\d+', substring)
        if digit_match:
            digits = int(digit_match.group()[::-1])
            max_num = max(max_num, digits)
    return max_num


with open('input.txt', 'r') as file:
    sum = 0
    for line in file:
        product = 1
        for color in colors:
            patterns = re.compile(color, flags=re.IGNORECASE)
            matches = patterns.finditer(line)
            product *= find_max_color(line, color, 0, matches)
        sum += product
        print(sum, product, line)

import re

with open('input.txt', 'r') as file:
    dad = 0
    for line in file:
        digit_pattern = re.compile(r'\d')
        digits = digit_pattern.findall(line)
        first_dig = digits[0]
        last_dig = digits[-1]
        cal_val = int(first_dig + last_dig)
        dad += cal_val
print(dad)

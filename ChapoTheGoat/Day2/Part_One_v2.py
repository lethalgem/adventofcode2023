import re
dad = 0
colors = ["blue", "red", "green"]
comparison_values = {"blue": 14, "red": 12, "green": 13}
with open('input.txt', 'r') as file:
    for line in file:
        digit_pattern = re.compile(r'\d+')
        possible_game_n = int(digit_pattern.findall(line)[0])
        skip_line = False
        for color in colors:
            if skip_line:
                break
            patterns = re.compile(color, flags=re.IGNORECASE)
            matches = patterns.finditer(line)
            for match in matches:
                start_index = match.start()
                end_index = match.end()
                substring = line[:start_index][::-1]
                digit_match = re.search(r'\d+', substring)
                if digit_match:
                    comparison_value = comparison_values.get(color, -1)
                    digits = int(digit_match.group()[::-1])
                    if digits > comparison_value:
                        skip_line = True
        if skip_line == False:
            dad += possible_game_n
    print(dad)
    # Way tooooo ez =D

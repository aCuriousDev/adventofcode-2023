def parse_game_data(line):
    game_id, data = line.split(': ')
    game_id = int(game_id.split(' ')[1])
    reveals = data.split('; ')

    game_reveals = []
    for reveal in reveals:
        counts = {'red': 0, 'green': 0, 'blue': 0}
        cubes = reveal.split(', ')
        for cube in cubes:
            parts = cube.split(' ')
            if len(parts) == 2:
                count, color = parts
                counts[color] = max(counts[color], int(count))
        game_reveals.append(counts)

    return game_id, game_reveals


def calculate_min_cubes(game_reveals):
    min_cubes = {'red': 0, 'green': 0, 'blue': 0}
    for reveal in game_reveals:
        for color in min_cubes:
            min_cubes[color] = max(min_cubes[color], reveal.get(color, 0))
    return min_cubes


def calculate_power(min_cubes):
    return min_cubes['red'] * min_cubes['green'] * min_cubes['blue']


def main(input_file):
    with open(input_file, 'r') as file:
        games = [parse_game_data(line.strip()) for line in file]

    total_power = 0
    for game_id, reveals in games:
        min_cubes = calculate_min_cubes(reveals)
        total_power += calculate_power(min_cubes)

    return total_power


input_file = 'input.txt'
print(main(input_file))

def parse_game_data(line):
    game_id, data = line.split(': ')
    game_id = int(game_id.split(' ')[1])
    reveals = data.split('; ')

    # Parse each reveal into a dictionary of cube counts
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


def is_game_possible(game_reveals, max_cubes):
    # Check if any reveal in the game exceeds the max cube counts
    for reveal in game_reveals:
        if any(reveal[color] > max_cubes[color] for color in max_cubes):
            return False
    return True


def main(input_file):
    with open(input_file, 'r') as file:
        games = [parse_game_data(line.strip()) for line in file]

    max_cubes = {'red': 12, 'green': 13, 'blue': 14}
    possible_games_sum = sum(
        game_id for game_id, reveals in games if is_game_possible(reveals, max_cubes))

    return possible_games_sum

input_file = 'input.txt'
print(main(input_file))

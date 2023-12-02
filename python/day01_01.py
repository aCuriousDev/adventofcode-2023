def sum_calibration_values(filename):
    total_sum = 0

    with open(filename, 'r') as file:
        for line in file:
            first_digit = next((char for char in line if char.isdigit()), None)
            last_digit = next(
                (char for char in reversed(line) if char.isdigit()), None)

            if first_digit and last_digit:
                calibration_value = int(first_digit + last_digit)
                total_sum += calibration_value

    return total_sum


filename = "../input"
total_sum = sum_calibration_values(filename)
print(f"Total sum of calibration values: {total_sum}")

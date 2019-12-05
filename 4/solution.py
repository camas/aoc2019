from itertools import groupby


def solve(data: str) -> str:
    # Parse input
    range_min, range_max = [int(a) for a in data.splitlines()[0].split('-')]

    passwords_found = 0  # Part 1
    proper_passwords_found = 0  # Part 2
    for i in range(range_min, range_max):
        # Split i into digits
        digits = [int(c) for c in str(i)]

        # Group digits by similar
        digits = [(key, len(list(value))) for key, value in groupby(digits)]

        # Check digits match conditions
        pair_exists = False
        two_same = False
        decrease = False
        prev_digit = -1
        for digit, count in digits:
            if digit < prev_digit:
                decrease = True
                break
            if count >= 2:
                two_same = True
            if count == 2:
                pair_exists = True

            prev_digit = digit

        # Update counts
        if not decrease and two_same:
            passwords_found += 1
            if pair_exists:
                proper_passwords_found += 1

    return f"{passwords_found} {proper_passwords_found}"

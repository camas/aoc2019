def solve(data: str) -> str:
    initial_total = 0
    total = 0
    for line in data.splitlines():
        mass = int(line)
        initial_total += mass // 3 - 2

        curr = mass
        while True:
            fuel = curr // 3 - 2
            if fuel <= 0:
                break
            total += fuel
            curr = fuel
    return f"{initial_total} {total}"

PART_2_TARGET = 19690720


def solve(data: str):
    # Tad messy. lazy
    global values
    values = [int(c) for c in data.split(',')]

    # solve part 1
    part1 = run_inputs(12, 2)

    # Brute force part 2
    found = False
    for x in range(0, 100):
        for y in range(0, 100):
            result = run_inputs(x, y)
            if result == PART_2_TARGET:
                found = True
                part2 = (100 * x) + y
                break
        if found:
            break

    return f"{part1} {part2}"


def run_inputs(a: int, b: int) -> int:
    memory = values[:]
    memory[1] = a
    memory[2] = b
    pos = 0
    while True:
        opcode = memory[pos]
        if opcode == 99:
            break
        p = memory[pos + 1: pos + 4]
        if opcode == 1:
            memory[p[2]] = memory[p[0]] + memory[p[1]]
        elif opcode == 2:
            memory[p[2]] = memory[p[0]] * memory[p[1]]
        else:
            raise Exception()
        pos += 4

    return memory[0]

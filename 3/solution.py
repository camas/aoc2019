from typing import List, Tuple
from collections import defaultdict


def tests() -> List[Tuple[str, str]]:
    return [
        ("R8,U5,L5,D3\nU7,R6,D4,L4", "6 30"),
        ("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
            "159 610"),
        (("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n"
          "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), "135 410"),
    ]


def solve(data: str) -> str:
    # Parse wires
    lines = data.splitlines()
    wire1 = read_wire(lines[0])
    wire2 = read_wire(lines[1])

    # Find overlaps
    points = defaultdict(lambda: (0, 0))
    dist_dict = {
        'U': (0, 1),
        'D': (0, -1),
        'L': (-1, 0),
        'R': (1, 0),
    }
    total_dist = 0
    for x, y, direct, dist in wire1[:-1]:
        for i in range(1, dist + 1):
            total_dist += 1
            m = dist_dict[direct]
            p = (x + i * m[0], y + i * m[1])
            cur = points[p]
            points[p] = (total_dist, cur[1])
    total_dist = 0
    for x, y, direct, dist in wire2[:-1]:
        for i in range(1, dist + 1):
            total_dist += 1
            m = dist_dict[direct]
            p = (x + i * m[0], y + i * m[1])
            cur = points[p]
            points[p] = (cur[0], total_dist)

    # Find closest overlap
    best_dist = 9000000000000000000
    best_walk = 9000000000000000000
    for point, value in points.items():
        v1, v2 = value
        if not v1 or not v2:
            continue
        if point == (0, 0):
            continue
        dist = abs(point[0]) + abs(point[1])
        if dist < best_dist:
            best_dist = dist

        walk = v1 + v2
        if walk < best_walk:
            best_walk = walk

    return f"{best_dist} {best_walk}"


def read_wire(line: str) -> List[Tuple[int, int, str, int]]:
    cur = (0, 0)
    points = []

    for part in line.split(','):
        direction = part[0]
        dist = int(part[1:])

        if direction == 'U':
            new = (cur[0], cur[1] + dist)
        elif direction == 'D':
            new = (cur[0], cur[1] - dist)
        elif direction == 'R':
            new = (cur[0] + dist, cur[1])
        elif direction == 'L':
            new = (cur[0] - dist, cur[1])
        else:
            raise Exception()

        points.append((*cur, direction, dist))
        cur = new
    points.append((*cur, 'END', 0))

    return points

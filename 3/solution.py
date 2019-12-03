from typing import List, Tuple
from collections import defaultdict


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

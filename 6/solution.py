def tests():
    return [
        ("""COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN""", "54 4")
    ]


class Node:
    def __init__(self):
        self._parent = None


def solve(data: str) -> str:
    nodes = {}
    for line in data.splitlines():
        a, b = line.split(')')
        if a in nodes:
            na = nodes[a]
        else:
            na = Node()
            nodes[a] = na
        if b in nodes:
            nb = nodes[b]
        else:
            nb = Node()
            nodes[b] = nb
        nb._parent = na

    # Part 1
    # Brute force
    # From every node count back to root
    orbits = 0
    for n in nodes.values():
        cur = n
        while True:
            if not cur._parent:
                break
            orbits += 1
            cur = cur._parent

    # Part 2
    # walk back from both nodes until matching parent found
    start = nodes['YOU']._parent
    end = nodes['SAN']._parent
    walk_start = []
    cur = start
    while cur._parent:
        cur = cur._parent
        walk_start.append(cur)
    walk_end = []
    cur = end
    while cur._parent:
        cur = cur._parent
        walk_end.append(cur)

    for i, node in enumerate(walk_start, 1):
        if node in walk_end:
            # Found match
            node_index = walk_end.index(node)
            transfers = i + node_index + 1
            break

    return f"{orbits} {transfers}"

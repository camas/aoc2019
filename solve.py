#!/usr/bin/env python

import argparse
import sys
import importlib
from pathlib import Path

import pyperclip


def main():
    parser = argparse.ArgumentParser(description="Solve AoC 2019 puzzles")
    parser.add_argument('puzzles', metavar='P', type=int, nargs='+')

    if len(sys.argv) == 1:
        parser.print_help()
        sys.exit(1)

    args = parser.parse_args()

    for puzzle in args.puzzles:
        solver = importlib.import_module(f"{puzzle}.solution")
        if "tests" in dir(solver):
            tests = solver.tests()
            print(f"Running {len(tests)} tests")
            fail = False
            for i, test in enumerate(tests, 1):
                test_input, test_solution = test
                if solver.solve(test_input) != test_solution:
                    print(f"Failed test {i}")
                    fail = True
                else:
                    print(f"Test {i} passed")
            if fail:
                raise Exception("Tests didn't pass!")

        answer = solver.solve(read_input(puzzle))
        print(f"Solution to {puzzle} is {answer}")
        if len(args.puzzles) == 1:
            pyperclip.copy(answer)
            print("Copied to clipboard")


def read_input(puzzle: int) -> str:
    file = Path(str(puzzle)) / "input.txt"
    with open(file) as f:
        return f.read()


if __name__ == "__main__":
    main()

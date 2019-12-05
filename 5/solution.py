from typing import List
from enum import Enum


def solve(data: str) -> str:
    values = [int(c) for c in data.split(',')]

    # Run diagnostic
    machine = Machine(values)
    inp = [1]
    output = []
    machine.run(lambda: inp.pop(), lambda x: output.append(x))
    part1 = output[-1]

    # Run test 5
    machine = Machine(values)
    inp = [5]
    output = []
    machine.run(lambda: inp.pop(), lambda x: output.append(x))
    part2 = output[0]

    return f"{part1} {part2}"


class Machine:

    def __init__(self, inital_memory: List[int]):
        self._memory = inital_memory[:]

    def run(self, input_func, output_func):
        ip = 0  # Instruction pointer

        # Number of values each instruction takes
        instruction_sizes = {
            Instr.HALT: 0,
            Instr.ADD: 3,
            Instr.TIMES: 3,
            Instr.INPUT: 1,
            Instr.OUTPUT: 1,
            Instr.JUMP_IF_TRUE: 2,
            Instr.JUMP_IF_FALSE: 2,
            Instr.LESS_THAN: 3,
            Instr.EQUAL: 3,
        }

        # Run loop
        while True:
            # Parse instruction
            raw_instr = self._memory[ip]
            # Extra padding in case larger instructions are added
            str_instr = str(raw_instr)
            instr = Instr(int(str_instr[-2:]))
            instr_size = instruction_sizes[instr]
            modes = [int(c) for c in str_instr.rjust(instr_size + 2, '0')[:-2]]
            modes = list(reversed(modes))
            data = self._memory[ip+1:ip+1+instruction_sizes[instr]]

            # Helper functions
            def get_val(index: int) -> int:
                mode = modes[index]
                val = data[index]
                if mode == 0:
                    return self._memory[val]
                if mode == 1:
                    return val
                raise Exception("Unknown mode")

            def set_val(index: int, value: int) -> None:
                mode = modes[index]
                val = data[index]
                if mode == 0:
                    self._memory[val] = value
                elif mode == 1:
                    raise Exception("Can't write in immidiate mode")
                else:
                    raise Exception("Unknown mode")

            # Run instruction
            if instr == Instr.HALT:
                break
            elif instr == Instr.ADD:
                val = get_val(0) + get_val(1)
                set_val(2, val)
            elif instr == Instr.TIMES:
                val = get_val(0) * get_val(1)
                set_val(2, val)
            elif instr == Instr.INPUT:
                val = input_func()
                set_val(0, val)
            elif instr == Instr.OUTPUT:
                val = get_val(0)
                output_func(val)
            elif instr == Instr.JUMP_IF_TRUE:
                if get_val(0) != 0:
                    ip = get_val(1)
                    continue
            elif instr == Instr.JUMP_IF_FALSE:
                if get_val(0) == 0:
                    ip = get_val(1)
                    continue
            elif instr == Instr.LESS_THAN:
                if get_val(0) < get_val(1):
                    set_val(2, 1)
                else:
                    set_val(2, 0)
            elif instr == Instr.EQUAL:
                if get_val(0) == get_val(1):
                    set_val(2, 1)
                else:
                    set_val(2, 0)
            else:
                raise Exception("Unknown opcode")

            # Increment ip
            ip += instr_size + 1

    def get_val(self, p: int, mode: int) -> int:
        if mode == 0:
            return self._memory[p]
        if mode == 1:
            return p
        raise Exception("Unknown mode")

    def set_val(self, p: int, mode: int, value: int) -> None:
        if mode == 0:
            self._memory[p] = value
        elif mode == 1:
            raise Exception("Can't write in immidiate mode")
        else:
            raise Exception("Unknown mode")


class Instr(Enum):
    HALT = 99
    ADD = 1
    TIMES = 2
    INPUT = 3
    OUTPUT = 4
    JUMP_IF_TRUE = 5
    JUMP_IF_FALSE = 6
    LESS_THAN = 7
    EQUAL = 8

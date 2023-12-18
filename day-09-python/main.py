from pathlib import Path
from dataclasses import dataclass


def test_part1():
    fname = Path(__file__).parent / "data" / "test_input"
    assert solve_part1(fname) == 114


@dataclass
class Row:
    values: list[int]
    child: "Row | None"

    @classmethod
    def create(cls, row: list[int]):
        if all([v == 0 for v in row]):
            return cls(values=row, child=None)
        child = cls.create(diff(row))
        return cls(values=row, child=child)

    def extrapolate(self):
        if self.child is None:
            value = 0
        else:
            self.child.extrapolate()
            value = self.values[-1] + self.child.values[-1]
        self.values.append(value)


def diff(values):
    diff = [values[i + 1] - values[i] for i in range(len(values) - 1)]
    return diff


def solve_part1(fname: Path) -> int:
    with open(fname, "r") as f:
        input = [[int(n) for n in line.split()] for line in f]
    trees = [Row.create(row) for row in input]
    result = 0
    for tree in trees:
        tree.extrapolate()
        result += tree.values[-1]
    return result


if __name__ == "__main__":
    fname = Path(__file__).parent / "data" / "input"
    result = solve_part1(fname)
    print(f"Solution to part 1: {result}")

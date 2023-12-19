import pytest
from pathlib import Path
from dataclasses import dataclass


class TestCondition:
    def test_invalid_with_unknowns(self):
        condition = Condition("##.?.?", [2, 1, 1])
        with pytest.raises(ValueError):
            condition.is_valid()

    def test_valid(self):
        condition = Condition("...##..#..#.", [2, 1, 1])
        assert condition.is_valid()

    def test_invalid(self):
        condition = Condition("...#...#..#.", [2, 1, 1])
        assert not condition.is_valid()

    def test_n_arrangements_trivial(self):
        condition = Condition("????.#...#...", [4, 1, 1])
        assert condition.n_arrangements == 1

    def test_n_arrangements_1(self):
        condition = Condition("????.######..#####.", [1, 6, 5])
        assert condition.n_arrangements == 4

    def test_n_arrangements_2(self):
        condition = Condition("?###????????", [3, 2, 1])
        assert condition.n_arrangements == 10


def test_part1():
    fname = Path(__file__).parent / "data" / "test_input"
    assert solve_part1(fname) == 21


@dataclass
class Condition:
    record: str
    damaged_groups: list[int]

    def __repr__(self) -> str:
        return str(self)

    def __str__(self) -> str:
        goal = ",".join([str(g) for g in self.damaged_groups])
        return f"{self.record} {goal}"

    def is_valid(self) -> bool:
        if "?" in self.record:
            raise ValueError(
                f"Cannot check if condition is valid because it has unknowns: '{self}'"
            )
        record = [s for s in self.record.split(".") if s]
        counts = [s.count("#") for s in record]
        return counts == self.damaged_groups

    @property
    def n_arrangements(self) -> int:
        """Return number of possible arrangements"""
        n_arrangenments = 0
        for new_record in self._possible_records():
            if new_record.is_valid():
                n_arrangenments += 1
        return n_arrangenments

    def _possible_records(self):
        n_unknowns = self.record.count("?")
        n_damaged = self.record.count("#")
        required_damaged = sum(self.damaged_groups)
        free_damaged = required_damaged - n_damaged
        free_operational = n_unknowns - free_damaged
        free_elements = ["#"] * free_damaged + ["."] * free_operational
        return (self._generate_record(p) for p in permutations_unique(free_elements))

    def _generate_record(self, free_elements):
        record = [c for c in self.record]
        indices_unknowns = [i for i, char in enumerate(record) if char == "?"]
        for index, element in zip(indices_unknowns, free_elements):
            record.pop(index)
            record.insert(index, element)
        return Condition("".join(record), self.damaged_groups)


def permutations_unique(iterable):
    """
    Generator for all possible permutations without duplicates
    """
    if len(iterable) == 1:
        yield iterable
    for i in range(len(iterable)):
        if iterable[i] not in iterable[:i]:
            a = list(iterable)
            a[0], a[i] = a[i], a[0]
            for p in permutations_unique(a[1:]):
                yield a[:1] + p


def solve_part1(fname):
    result = 0
    with open(fname, "r") as f:
        for line in f:
            record, damaged_groups = line.split()
            damaged_groups = [int(s) for s in damaged_groups.split(",")]
            condition = Condition(record, damaged_groups)
            result += condition.n_arrangements
    return result


if __name__ == "__main__":
    fname = Path(__file__).parent / "data" / "input"
    result = solve_part1(fname)
    print(f"Solution to part 1: {result}")

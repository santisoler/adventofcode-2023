"""
More efficient solution to day 12

Based on the code of HyperNeutrino: https://www.youtube.com/watch?v=g3Ms5e7Jdqo

It took me more time that I would like to admit to understand this solution.
Thanks to HyperNeutrino for making a nice explained video with a super clean
code.
"""
from functools import cache
import pytest
from pathlib import Path


@pytest.mark.parametrize(
    "springs, hints, expected",
    [
        ["?##", (3,), 1],
        ["??#", (3,), 1],
        ["???", (3,), 1],
        ["?#??..#", (3, 1), 2],
        ["?#??.#", (3, 1), 2],
    ],
)
def test_possible_arrangements(springs, hints, expected):
    assert n_arrangements(springs, hints) == expected


def test_part1():
    fname = Path(__file__).parent / "data" / "test_input"
    assert solve_part1(fname) == 21


def test_part2():
    fname = Path(__file__).parent / "data" / "test_input"
    assert solve_part2(fname) == 525152


@cache
def n_arrangements(springs: str, hints: tuple[int] | tuple) -> int:
    # Base behaviours:
    #   Found EOL
    if not springs:
        return 1 if not hints else 0
    #   No more hints left
    if not hints:
        return 1 if "#" not in springs else 0

    result = 0

    # Deal with an operational spring and treat any '?' as one of those
    if springs[0] in (".", "?"):
        result += n_arrangements(springs[1:], hints)

    # Deal with a damaged spring and treat any '?' as one of those
    if springs[0] in ("#", "?"):
        if is_next_group_valid(springs, hints[0]):
            # We need to add a "+ 1" to avoid allowing the recursion to
            # start a block right next to the current one
            result += n_arrangements(springs[hints[0] + 1 :], hints[1:])
    return result


def is_next_group_valid(springs: str, hint: int) -> bool:
    """
    Determine if the next group is valid

    The next group might be invalid if:

      - There are less springs left that ``hint``
      - There is any operational (``"."``) spring in the next ``hint`` springs.
      - The next spring after ``hint`` springs (if there is one) is not
        damaged (``"#"``).
    """
    if len(springs) < hint:
        return False
    if "." in springs[:hint]:
        return False
    if len(springs) > hint and springs[hint] == "#":
        return False
    return True


def solution(fname: Path, unfold: bool):
    result = 0
    with open(fname, "r") as f:
        for line in f:
            springs, hints_str = line.split()
            hints = tuple(int(s) for s in hints_str.split(","))
            if unfold:
                springs = "?".join(5 * [springs])
                hints = 5 * hints
            result += n_arrangements(springs, hints)
    return result


def solve_part1(fname):
    return solution(fname, unfold=False)


def solve_part2(fname):
    return solution(fname, unfold=True)


if __name__ == "__main__":
    fname = Path(__file__).parent / "data" / "input"
    result = solve_part1(fname)
    print(f"Solution to part 1: {result}")
    result = solve_part2(fname)
    print(f"Solution to part 2: {result}")

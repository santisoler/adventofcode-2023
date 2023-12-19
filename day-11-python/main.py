import itertools
from pathlib import Path


def test_part1():
    fname = Path(__file__).parent / "data" / "test_input"
    assert solve_part1(fname) == 374


def parse_file(fname):
    galaxies = []
    with open(fname, "r") as f:
        galaxies = [
            (i, j)
            for j, line in enumerate(f)
            for i, char in enumerate(line)
            if char == "#"
        ]
    return galaxies


def expand(galaxies):
    cols = sorted(list(set([g[0] for g in galaxies])))
    rows = sorted(list(set([g[1] for g in galaxies])))
    missing_cols = [col for col in range(min(cols), max(cols)) if col not in cols]
    missing_rows = [row for row in range(min(rows), max(rows)) if row not in rows]
    expanded_galaxies = []
    for galaxy in galaxies:
        x, y = galaxy
        n_additional_cols = len([m for m in missing_cols if m < x])
        n_additional_rows = len([m for m in missing_rows if m < y])
        expanded_galaxies.append((x + n_additional_cols, y + n_additional_rows))
    return expanded_galaxies


def distance(galaxy_start, galaxy_end):
    return abs(galaxy_start[0] - galaxy_end[0]) + abs(galaxy_start[1] - galaxy_end[1])


def solve_part1(fname):
    galaxies = parse_file(fname)
    galaxies = expand(galaxies)
    pool = itertools.combinations(galaxies, 2)
    distances = [distance(g1, g2) for g1, g2 in pool]
    return sum(distances)


if __name__ == "__main__":
    fname = Path(__file__).parent / "data" / "input"
    result = solve_part1(fname)
    print(result)

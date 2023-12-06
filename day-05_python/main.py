from pathlib import Path
from dataclasses import dataclass


class Tests:
    def test_map(self):
        map = Map([(50, 98, 2), (52, 50, 48)])
        assert map.get(0) == 0
        assert map.get(1) == 1
        assert map.get(48) == 48
        assert map.get(49) == 49
        assert map.get(50) == 52
        assert map.get(51) == 53
        assert map.get(96) == 98
        assert map.get(97) == 99
        assert map.get(98) == 50
        assert map.get(99) == 51

    def test_maps(self):
        fname = Path(__file__).parent / Path("../day-05/data/test_input")
        seeds, maps = parse_input(fname)
        expected_locations = [82, 43, 86, 35]
        for seed, location in zip(seeds, expected_locations):
            assert get_location(seed, maps) == location

    def test_part1(self):
        fname = Path(__file__).parent / Path("../day-05/data/test_input")
        assert solve_part1(fname) == 35


@dataclass
class Map:
    map: list[tuple[int, int, int]]

    @classmethod
    def new(cls):
        return cls([])

    def append(self, dest: int, source: int, len: int) -> None:
        self.map.append((dest, source, len))

    def get(self, value: int) -> int:
        for dest, source, len in self.map:
            if source <= value < source + len:
                return (value - source) + dest
        return value


def parse_input(fname: str | Path) -> tuple[list[int], list[Map]]:
    maps = []
    eof = False
    with open(fname, "r") as f:
        seeds = [int(s.strip()) for s in f.readline().split()[1:]]
        # Read empty line
        f.readline()
        # Start reading maps
        while True:
            # Define new map
            map = Map.new()
            # Read title
            line = f.readline()
            # Read first line
            line = f.readline()
            while line != "\n":
                dest, source, len = tuple(int(s) for s in line.split())
                map.append(dest, source, len)
                line = f.readline()
                if line == "":
                    eof = True
                    break
            maps.append(map)
            if eof:
                break
    return seeds, maps


def get_location(seed: int, maps: list[Map]) -> int:
    for map in maps:
        seed = map.get(seed)
    return seed


def solve_part1(fname) -> int:
    seeds, maps = parse_input(fname)
    min_location = min([get_location(seed, maps) for seed in seeds])
    return min_location


if __name__ == "__main__":
    fname = Path(__file__).parent / Path("../day-05/data/input")
    result = solve_part1(fname)
    print(f"Solution to part 1: {result}")

from pathlib import Path
from dataclasses import dataclass

MAX_RED, MAX_GREEN, MAX_BLUE = 12, 13, 14


@dataclass
class Set:
    red: int
    green: int
    blue: int

    @property
    def is_possible(self) -> bool:
        if (self.red > MAX_RED) or (self.green > MAX_GREEN) or (self.blue > MAX_BLUE):
            return False
        return True


@dataclass
class Game:
    id: int
    sets: list[Set]

    @property
    def is_possible(self) -> bool:
        if any([not s.is_possible for s in self.sets]):
            return False
        return True

    def get_minimum_set(self) -> Set:
        """
        Return minimum set needed to play this game
        """
        red = max([s.red for s in self.sets])
        green = max([s.green for s in self.sets])
        blue = max([s.blue for s in self.sets])
        return Set(red=red, green=green, blue=blue)


def parse_line(line: str) -> Game:
    """
    Parse single line: containing a single game
    """
    game_info, sets_str = line.split(":")
    game_id = int(game_info.split()[-1].replace(":", ""))
    sets_list = []
    for sets in sets_str.split(";"):
        red, blue, green = 0, 0, 0
        for cubes in sets.split(","):
            n_cubes_str, color = cubes.strip().split()
            n_cubes = int(n_cubes_str)
            match color:
                case "red":
                    red = n_cubes
                case "green":
                    green = n_cubes
                case "blue":
                    blue = n_cubes
                case other:
                    raise ValueError(f"Invalid color '{other}'")
            sets_list.append(Set(red=red, blue=blue, green=green))
    return Game(id=game_id, sets=sets_list)


def solve_part1(fname) -> int:
    result = 0
    with open(fname, "r") as f:
        for line in f:
            game = parse_line(line)
            if game.is_possible:
                result += game.id
    return result


def solve_part2(fname) -> int:
    result = 0
    with open(fname, "r") as f:
        for line in f:
            game = parse_line(line)
            minimum_set = game.get_minimum_set()
            result += minimum_set.red * minimum_set.green * minimum_set.blue
    return result


def test_part1():
    fname = Path("..") / "day-02" / "data" / "test_input"
    assert solve_part1(fname) == 8


def test_part2():
    fname = Path("..") / "day-02" / "data" / "test_input"
    assert solve_part2(fname) == 2286


if __name__ == "__main__":
    fname = Path("..") / "day-02" / "data" / "input"
    result = solve_part1(fname)
    print(f"Solution to part 1: {result}")
    result = solve_part2(fname)
    print(f"Solution to part 2: {result}")

from pathlib import Path


def test_part1():
    fname = Path(__file__).parent / "data" / "test_input"
    assert solve_part1(fname) == 405


def test_part2():
    fname = Path(__file__).parent / "data" / "test_input"
    assert solve_part2(fname) == 400


def read_file(fname: Path) -> list[list[list[bool]]]:
    arrays = []
    eof = False
    with open(fname, "r") as f:
        while not eof:
            pattern = []
            while True:
                line = f.readline()
                if line == "\n":
                    break
                elif line == "":
                    eof = True
                    break
                row = [char_to_bool(c) for c in line.replace("\n", "")]
                pattern.append(row)
            arrays.append(pattern)
    return arrays


def char_to_bool(char: str) -> bool:
    if char == "#":
        return True
    if char == ".":
        return False
    raise ValueError(f"Invalid character {char}")


def is_palindrome(sequence) -> bool:
    """Check if the given sequence is a palyndrome."""
    # Check sequence has even number of elements
    if len(sequence) % 2 != 0:
        raise ValueError("Sequence with odd number of elements")
    return _is_palindrome(sequence)


def _is_palindrome(sequence):
    """Recursive function to check if a given sequence is a palindrome."""
    if len(sequence) == 2:
        return sequence[0] == sequence[1]
    if sequence[0] != sequence[-1]:
        return False
    return _is_palindrome(sequence[1:-1])


def find_symmetry_plane(pattern, vertical=True) -> int | None:
    if not vertical:
        pattern = transpose(pattern)

    length = len(pattern[0])
    planes = [i for i in range(1, length)]

    for row in pattern:
        if not planes:
            break
        i = 0
        while i < len(planes):
            plane = planes.pop(i)
            start, end = get_start_end(plane, length)
            if is_palindrome(row[start:end]):
                planes.insert(i, plane)
                i += 1

    if not planes:
        return None
    if len(planes) > 1:
        raise ValueError(f"Encountered several symmetry planes: '{planes}'")
    return planes[0]


def pattern_to_string(pattern):
    string = ""
    for row in pattern:
        string += "".join(["#" if i else "." for i in row]) + "\n"
    return string


def find_almost_symmetry_plane(pattern, vertical=True) -> int | None:
    if not vertical:
        pattern = transpose(pattern)

    n_cols = len(pattern[0])

    result = None
    for plane in range(1, n_cols):
        start, end = get_start_end(plane, n_cols)
        p_counts = [count_non_palindromy(row[start:end]) for row in pattern]
        if sum(p_counts) == 1:
            result = plane
            break
    return result


def count_non_palindromy(sequence):
    """
    Count how many pairs of elements in the sequence doesn't satisfy palindromy.
    """
    comparisons = [sequence[i] != sequence[-(i + 1)] for i in range(len(sequence) // 2)]
    return sum(comparisons)


def transpose(array: list[list]) -> list[list]:
    return [list(x) for x in zip(*array)]


def get_start_end(plane: int, length: int) -> tuple[int, int]:
    if plane == 0:
        raise ValueError("Invalid plane zero")
    if plane == length:
        raise ValueError(f"Invalid plane {plane}")
    delta = min(plane, length - plane)
    start = plane - delta
    end = plane + delta
    return start, end


def solve_part1(fname: Path) -> int:
    patterns = read_file(fname)
    result = 0
    for pattern in patterns:
        symmetry_plane = find_symmetry_plane(pattern, vertical=True)
        if symmetry_plane is not None:
            result += symmetry_plane
            continue

        symmetry_plane = find_symmetry_plane(pattern, vertical=False)
        if symmetry_plane is not None:
            result += 100 * symmetry_plane
        else:
            raise RuntimeError("Unable to find any symmetry plane")

    return result


def solve_part2(fname: Path) -> int:
    patterns = read_file(fname)
    result = 0
    for pattern in patterns:
        symmetry_plane = find_almost_symmetry_plane(pattern, vertical=True)
        if symmetry_plane is not None:
            result += symmetry_plane
            continue

        symmetry_plane = find_almost_symmetry_plane(pattern, vertical=False)
        if symmetry_plane is not None:
            result += 100 * symmetry_plane
        else:
            raise RuntimeError("Unable to find any symmetry plane")

    return result


if __name__ == "__main__":
    fname = Path(__file__).parent / "data" / "input"
    result = solve_part1(fname)
    print(f"Solution to part 1: {result}")
    result = solve_part2(fname)
    print(f"Solution to part 2: {result}")

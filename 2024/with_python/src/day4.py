def part1(content: list[str]) -> int:
    # fmt: off
    DIRECTIONS = [
        (-1, -1), (-1, +0), (-1, +1),
        (+0, -1),           (+0, +1),
        (+1, -1), (+1, +0), (+1, +1),
    ]
    # fmt: on

    h = len(content)
    w = len(content[0])

    def is_in_bounds(x: int, y: int) -> bool:
        return 0 <= x < w and 0 <= y < h

    def check_xmas(x: int, y: int, h_dir: int, v_dir: int) -> bool:
        positions = [(x + i * h_dir, y + i * v_dir) for i in range(4)]

        if not all(is_in_bounds(x, y) for x, y in positions):
            return False
        target = "XMAS"
        return all(content[px][py] == target[i] for i, (px, py) in enumerate(positions))

    return sum(
        check_xmas(x, y, h_dir, v_dir)
        for x in range(h)
        for y in range(w)
        for h_dir, v_dir in DIRECTIONS
        if content[x][y] == "X"
    )


def part2(content: list[str]) -> int:
    def check_patterns(x: int, y: int) -> bool:
        if content[x][y] != "A":
            return False

        PATTERNS = [
            {"tl": "M", "bl": "M", "tr": "S", "br": "S"},
            {"tl": "S", "bl": "S", "tr": "M", "br": "M"},
            {"tl": "M", "bl": "S", "tr": "M", "br": "S"},
            {"tl": "S", "bl": "M", "tr": "S", "br": "M"},
        ]

        symbols_around_A = {
            "tl": content[x - 1][y - 1],
            "tr": content[x - 1][y + 1],
            "bl": content[x + 1][y - 1],
            "br": content[x + 1][y + 1],
        }

        return symbols_around_A in PATTERNS

    h = len(content)
    w = len(content[0])

    return sum(check_patterns(x, y) for x in range(1, h - 1) for y in range(1, w - 1))

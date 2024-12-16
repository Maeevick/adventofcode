# Think Stupid

## Run the code

```shell
poetry shell
poetry run python src/main.py
```

## Run the test suite

```shell
poetry shell
poetry run pytest
```

## Run black formatter
```shell
poetry run black SRC
```

## HOW TO (_it's more for me than real guidelines for anyone else_)

- Add one file `dayX` in the `src` folder

```python
def part1():
    return "This is day one, part one"

def part2():
    return "This is day one, part two"
```

- Add one file `test_dayX` in the `tests` folder

```python
from src.dayX import part1, part2

def test_dayX_part1_says():
    assert part1() == "This is day XXX, part one"

def test_dayX_part2_says():
    assert part2() == "This is day XXX, part two"
```

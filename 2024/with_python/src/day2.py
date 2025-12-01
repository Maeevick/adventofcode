def part1(content: list[str]) -> int:
    return sum(1 for report in content if is_sorted(report) and has_valid_gaps(report))


def part2(content: list[str]) -> int:
    return sum(
        1 for report in content if is_sorted_and_valid_gaps_with_exception(report)
    )


def is_sorted(report: str) -> bool:
    nums = [int(x) for x in report.split()]
    return nums == sorted(nums) or nums == sorted(nums, reverse=True)


def has_valid_gaps(report: str) -> bool:
    nums = [int(x) for x in report.split()]
    return all(1 <= abs(nums[i] - nums[i - 1]) <= 3 for i in range(1, len(nums)))


def is_sorted_and_valid_gaps_with_exception(report: str) -> bool:
    if is_sorted(report) and has_valid_gaps(report):
        return True

    nums = report.split()
    return any(
        is_sorted(" ".join(nums[:i] + nums[i + 1 :]))
        and has_valid_gaps(" ".join(nums[:i] + nums[i + 1 :]))
        for i in range(len(nums))
    )

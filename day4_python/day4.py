import re
from typing import Dict, Generator


def read_input() -> Generator[Dict[str, str], None, None]:
    passports = []
    with open("input.txt", "r") as f:
        passport = ""
        for line in f.readlines():
            if line == "\n":
                passports.append(passport)
                passport = ""
            else:
                passport += line

    for passport in passports:
        pairs = " ".join(passport.split("\n")).strip().split(" ")
        yield {pair.split(":")[0]: pair.split(":")[1] for pair in pairs}


def validate_passport_part_1(passport: Dict[str, str]) -> bool:
    return all(
        field in passport.keys()
        for field in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    )


def validate_height(height: str) -> bool:
    if "cm" in height:
        quantity = int(height[0 : height.find("cm")])
        return quantity >= 150 and quantity <= 193
    else:
        quantity = int(height[0 : height.find("in")])
        return quantity >= 59 and quantity <= 76


def validate_hair_color(color: str) -> bool:
    return re.match(r"^#[a-f0-9]{6}$", color) is not None


def validate_eye_color(color: str) -> bool:
    return color in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]


def validate_passport_id(pid: str) -> bool:
    return re.match(r"^\d{9}$", pid) is not None


def validate_passport_part_2(passport: Dict[str, str]) -> bool:
    year_in_range = (
        lambda year, min_year, max_year: int(year) >= min_year and int(year) <= max_year
    )
    return all(
        (
            year_in_range(passport["byr"], 1920, 2002),
            year_in_range(passport["iyr"], 2010, 2020),
            year_in_range(passport["eyr"], 2020, 2030),
            validate_height(passport["hgt"]),
            validate_hair_color(passport["hcl"]),
            validate_eye_color(passport["ecl"]),
            validate_passport_id(passport["pid"]),
        )
    )


if __name__ == "__main__":
    # Part 1
    print(
        len(
            [
                passport
                for passport in read_input()
                if validate_passport_part_1(passport)
            ]
        )
    )

    # Part 2
    print(
        len(
            [
                passport
                for passport in read_input()
                if validate_passport_part_1(passport)
                and validate_passport_part_2(passport)
            ]
        )
    )

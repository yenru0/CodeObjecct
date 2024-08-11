# I V X L C
ROMANS = [None] * 100

ROMANS_COUNTER = [None] * 100


def roman_counter(symbol: str) -> tuple:
    return tuple(symbol.count(char) for char in ("I", "V", "X", "L", "C"))


def roman_convertor(num: int) -> str:
    # fmt: off
    return ("", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC")[num // 10] \
         + ("", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX")[num % 10]
    # fmt: on


if __name__ == "__main__":
    roman = input()

    for i in range(100):
        temp = roman_convertor(i)
        ROMANS[i] = temp
        ROMANS_COUNTER[i] = roman_counter(temp)

    count = roman_counter(roman)

    for i in range(100):
        if ROMANS_COUNTER[i] == count:
            print(ROMANS[i])
            break

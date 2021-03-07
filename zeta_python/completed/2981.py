def divisor(x):
    divs = []
    for i in range(1, int(x ** (1 / 2)) + 1):
        if x % i == 0:
            divs.append(i)
    for d in divs[::-1]:
        t = x // d
        if d == t:
            continue
        divs.append(x // d)
    return divs


def gcd(x1, x2):
    while x2 != 0:
        temp = x1 % x2
        x1 = x2
        x2 = temp
    return x1


def solve(N, M):
    M.sort()
    last_night = M[1] - M[0]
    for i in range(1, N - 1):
        last_night = gcd(last_night, M[i + 1] - M[i])
    return " ".join(map(str, divisor(last_night)[1:]))


if __name__ == '__main__':
    n = int(input())
    print(solve(n, list(map(int, (input() for i in range(n))))))

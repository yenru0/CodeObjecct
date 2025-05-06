def solve(N):
    if N == 0 or N == 1:
        return 0
    count5 = 0

    for i in range(5, N + 1, 5):
        while i % 5 == 0:
            if i % 5 == 0:
                i //= 5
                count5 += 1

    return count5


if __name__ == '__main__':
    print(solve(int(input())))

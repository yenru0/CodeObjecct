def solve(N):
    if N == 0 or N == 1:
        return 0
    count2 = 0
    count5 = 0

    for i in range(2, N + 1):
        while i % 2 == 0 or i % 5 == 0:
            if i % 2 == 0:
                i //= 2
                count2 += 1
            elif i % 5 == 0:
                i //= 5
                count5 += 1

    return min((count2, count5))


if __name__ == '__main__':
    print(solve(int(input())))

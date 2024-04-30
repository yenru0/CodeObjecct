import sys

input = sys.stdin.readline
print = sys.stdout.write

N = int(input())


def solve(N: int) -> int:
    count = 0
    D = []
    for i in range(N):
        T = [i]
        D.append((1, T))

    while D:
        now, xs = D.pop()
        if now == N:
            count += 1
            continue

        for i in range(N):
            if i in xs:
                continue

            flag = True
            for j in range(now):
                y = xs[j]
                if abs(now - j) == abs(y - i):
                    flag = False
                    break
            if flag:
                D.append((now + 1, xs + [i]))

    return count


if __name__ == "__main__":
    print(str(solve(N)))

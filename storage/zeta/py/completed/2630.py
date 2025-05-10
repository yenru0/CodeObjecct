import sys

input = sys.stdin.readline


N: int  # late-init


def partition(L, P):
    """get subdivisions of partition and check is valid of its subdivision"""
    half = L // 2

    quad = [[], [], [], []]

    for i in range(L):
        for j in range(L):
            if 0 <= i < half:
                if 0 <= j < half:
                    quad[0].append(P[i * L + j])
                else:
                    quad[1].append(P[i * L + j])
            else:
                if 0 <= j < half:
                    quad[2].append(P[i * L + j])
                else:
                    quad[3].append(P[i * L + j])
    ret = []
    for q in quad:
        a = not any(q)
        b = all(q)
        if a:
            ret.append((True, 0))
        elif b:
            ret.append((True, 1))
        else:
            ret.append((False, half, q))
    return ret


def solve(N: int, M: list[int]):
    """main DFS process"""
    cnt_0 = 0
    cnt_1 = 0
    if all(M):
        return 0, 1
    elif not any(M):
        return 1, 0

    D = [(N, M)]  # stack for DFS

    while D:
        L, P = D.pop()
        parted = partition(L, P)
        for p in parted:
            if p[0]:
                _, cnt = p
                if cnt:
                    cnt_1 += 1
                else:
                    cnt_0 += 1
            else:
                _, new_L, new_P = p
                D.append((new_L, new_P))

    return cnt_0, cnt_1


if __name__ == "__main__":
    N = int(input())
    print(
        *solve(N, sum([list(map(int, input().split())) for _ in range(N)], [])),
        sep="\n"
    )

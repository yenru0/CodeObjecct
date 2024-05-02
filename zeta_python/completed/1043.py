import sys

input = sys.stdin.readline


def solve(N, M, know: set[int], parties):
    valid_party = [1 for _ in parties]

    flag = True

    while flag:
        flag = False
        for i, p in enumerate(parties):
            cond = [m in know for m in p]
            if all(cond):
                valid_party[i] = 0
            elif any(cond):
                flag = True
                valid_party[i] = 0
                for m in p:
                    know.add(m)

    return sum(valid_party)


if __name__ == "__main__":
    N, M = map(int, input().split())
    know = set()

    pre_know = list(map(int, input().split()))
    L = pre_know[0]
    for i in pre_know[1:]:
        know.add(i)

    parties = [list(map(int, input().split()))[1:] for _ in range(M)]

    print(solve(N, M, know, parties))

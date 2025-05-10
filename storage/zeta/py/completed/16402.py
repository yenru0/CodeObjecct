import sys

input = sys.stdin.readline


def find(deps, country) -> int:
    a = country
    while a != deps[a]:
        a = deps[a]
    return a


def union(deps, win, lose):
    rw, rl = find(deps, win), find(deps, lose)
    if rw == rl:
        if rl == lose:
            deps[rl] = win
            deps[win] = win
    else:
        deps[rl] = rw


def get_sovereigns(N, deps):
    return [i for i in range(N) if i == deps[i]]


if __name__ == "__main__":
    N, M = map(int, input().split())
    name = [input().split()[2] for _ in range(N)]
    name.sort()
    name2index = {name[i]: i for i in range(N)}
    index2name = [name[i] for i in range(N)]

    deps = [i for i in range(N)]

    for _ in range(M):
        string_a, string_b, result = input().split(",")
        name_a, name_b = string_a.split()[2], string_b.split()[2]
        ia, ib = name2index[name_a], name2index[name_b]
        result = int(result)
        union(deps, ia, ib) if result == 1 else union(deps, ib, ia)

    sovereigns = get_sovereigns(N, deps)
    print(len(sovereigns))
    [print(f"Kingdom of {index2name[i]}") for i in sovereigns]

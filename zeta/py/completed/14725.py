import sys

input = sys.stdin.readline

MAX = 150505


def print_structrue(N, S):
    trie = [dict() for _ in range(MAX)]

    cnt = 0

    for s in S:
        idx = 0
        for w in s:
            if trie[idx].get(w, 0):
                idx = trie[idx][w]
            else:
                trie[idx][w] = cnt + 1
                idx = cnt + 1
                cnt += 1

    D = [(0, 0, None)]
    while D:
        idx, depth, word = D.pop()
        if depth:
            print("--" * (depth - 1) + word)
        for key, value in sorted(trie[idx].items())[::-1]:
            D.append((value, depth + 1, key))


if __name__ == "__main__":
    N = int(input())
    S = [input().rstrip().split()[1:] for _ in range(N)]
    print_structrue(N, S)

import sys

input = sys.stdin.readline


def get_max_xor(N, S):
    trie = [[0, 0] for _ in range(3000000)]

    cnt = 0
    for s in S:
        idx = 0
        for b in s:
            b = int(b)
            if trie[idx][b]:
                idx = trie[idx][b]
            else:
                trie[idx][b] = cnt + 1
                idx = cnt + 1
                cnt += 1
    n = []
    D = [(0, 0, 0, 0)]
    while D:
        depth, idx1, idx2, xor = D.pop()
        if depth == 30:
            n.append(xor)

        flag = False

        if trie[idx1][0] and trie[idx2][1]:
            D.append(
                (depth + 1, trie[idx1][0], trie[idx2][1], xor + (1 << (29 - depth)))
            )
            flag = True
        if trie[idx1][1] and trie[idx2][0]:
            D.append(
                (depth + 1, trie[idx1][1], trie[idx2][0], xor + (1 << (29 - depth)))
            )
            flag = True

        if flag:
            continue

        if trie[idx1][0] and trie[idx2][0]:
            D.append((depth + 1, trie[idx1][0], trie[idx2][0], xor))
        if trie[idx1][1] and trie[idx2][1]:
            D.append((depth + 1, trie[idx1][1], trie[idx2][1], xor))
    return max(n)


if __name__ == "__main__":
    N = int(input())
    S = list(map(lambda x: format(int(x), "b").zfill(30), input().split()))
    print(get_max_xor(N, S))

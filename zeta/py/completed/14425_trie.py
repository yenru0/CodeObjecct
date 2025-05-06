import sys

input = sys.stdin.readline

trie = [[0 for _ in range(26)] for _ in range(5050505)]
fins = [0 for _ in range(5050505)]
cnt = 1


def convert(c: str) -> int:
    return ord(c) - ord("a")


# trie
def insert(s: str):
    global cnt
    cur = 0
    for c in s:
        nxt = convert(c)
        if not trie[cur][nxt]:
            trie[cur][nxt] = cnt
            cnt += 1
        cur = trie[cur][nxt]
    fins[cur] = 1


def query(s):
    cur = 0
    for c in s:
        nxt = convert(c)
        if not trie[cur][nxt]:
            if s == "sundaycoding":
                print(c)
            return 0
        cur = trie[cur][nxt]
    return fins[cur]


if __name__ == "__main__":
    N, M = map(int, input().split())
    for _ in range(N):
        insert(input().strip())

    print(sum(query(input().strip()) for _ in range(M)))

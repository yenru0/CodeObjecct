import sys

input = sys.stdin.readline

def is_consist(N: int, S: list) -> bool:
    S.sort()
    trie = [[0] * 10 for _ in range(155050)]
    fin = [0] * 155050
    cnt = 0
    for s in S:
        idx = 0
        for c in s:
            if fin[idx]:
                return False
            if trie[idx][int(c)]:
                idx = trie[idx][int(c)]
            else:
                trie[idx][int(c)] = cnt + 1
                idx = cnt + 1
                cnt += 1
        fin[idx] = 1

    return True



if __name__ == '__main__':
    T = int(input())
    for _ in range(T):
        N = int(input())
        S = [input().rstrip() for _ in range(N)]
        print("YES" if is_consist(N, S) else "NO")
import sys

input = sys.stdin.readline

if __name__ == "__main__":
    T = int(input())
    for tc in range(1, T + 1):
        M, N, P = map(int, input().split())

        counts = [[0 for _ in range(M)] for _ in range(P)]
        scores = [[0 for _ in range(M)] for _ in range(P)]
        solved = [0 for _ in range(P)]
        for _ in range(N):
            p, m, t, j = input().split()
            p, t, j = int(p) - 1, int(t), int(j)
            m = ord(m) - 65
            if j == 0:
                counts[p][m] += 1
            else:
                if scores[p][m] != 0:
                    continue
                else:
                    solved[p] += 1
                    scores[p][m] = counts[p][m] * 20 + t

        sumscore = [(solved[p], -sum(scores[p]), p) for p in range(P)]
        sumscore.sort(reverse=True)
        print(f"Data Set {tc}:")
        for s in sumscore:
            print(f"{s[2] + 1} {s[0]} {-s[1]}")
        print()

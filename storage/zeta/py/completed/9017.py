import sys

input = sys.stdin.readline
if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        N = int(input())
        X = list(map(int, input().split()))

        counts = [0] * 200
        counts2 = [0] * 200
        last_fifth_index = [0] * 200
        S = [0] * 200
        for i in range(N):
            counts[X[i] - 1] += 1
        filtered = [i for i, v in filter(lambda x: x[1] >= 6, enumerate(counts))]
        score = 0
        for i in range(N):
            if X[i] - 1 in filtered:
                score += 1
                counts2[X[i] - 1] += 1
                if counts2[X[i] - 1] <= 4:
                    S[X[i] - 1] += score
                elif counts2[X[i] - 1] == 5:
                    last_fifth_index[X[i] - 1] = i + 1

        print(sorted([(S[i], last_fifth_index[i], i) for i in filtered])[0][2] + 1)

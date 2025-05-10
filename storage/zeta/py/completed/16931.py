N, M = map(int, input().split())
sqs = [[0] + list(map(int, input().split())) + [0] for i in range(N)]
sqs.insert(0, list(0 for i in range(M + 2)))
sqs.append(list(0 for i in range(M + 2)))

total = 2 * N * M
for i in range(N + 1):
    for j in range(M + 1):
        total += abs(sqs[i][j] - sqs[i][j + 1])
        total += abs(sqs[i][j] - sqs[i + 1][j])
print(total)

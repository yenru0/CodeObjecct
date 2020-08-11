N, K = map(int, input().split())
Ws = [tuple(map(int, input().split())) for i in range(N)]  # weight, value

DP = [[0] * N] * K

print(DP)


chess_set = [1, 1, 2, 2, 2, 8, ]

my_set = list(map(int, input().split()))

delta_set = []

for u, v in zip(chess_set, my_set):
    delta_set.append(u - v)

print(" ".join(map(str, delta_set)))

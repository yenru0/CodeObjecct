X, N = int(input()), int(input())
mul = lambda x: x[0] * x[1]
print("Yes") if sum(mul(list(map(int, input().split()))) for _ in range(N)) == X else print("No")

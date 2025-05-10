a, k = map(int, input().split())
print(1 if a == 1 else (a if k == 0 else (a if a % 2 else 1)))

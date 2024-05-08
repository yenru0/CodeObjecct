import sys

input = sys.stdin.readline
print = sys.stdout.write

N = int(input())
d = dict()
for _ in range(N):
    n, t = input().rstrip().split()
    if t.startswith("e"):
        d[n] = 1
    else:
        d[n] = 0
x = sorted([k for k, v in d.items() if v == 1])[::-1]
for i in x:
    print(i)
    print('\n')

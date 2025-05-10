N = int(input())
for i in sorted([list(map(int,input().split()))for i in range(N)]):
    print(' '.join(map(str, i)))
import sys
N = int(sys.stdin.readline())
K = [0]*10000
for i in range(N):
    K[int(sys.stdin.readline())-1] += 1
for i in range(1,10001):
    for k in range(K[i-1]):
        print(i)

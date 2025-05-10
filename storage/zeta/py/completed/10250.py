T = int(input())
for i in range(T):
    H, W, N = map(int, input().split())
    print("%d%02d"%((N-1)%H+1,(N-1)//H+1))
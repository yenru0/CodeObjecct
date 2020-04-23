def fact(x):
    temp = 1
    for i in range(1, x+1):
        temp *= i
    return temp
def comb(n, r):
    return fact(n)//(fact(n-r)*fact(r))
N = int(input())
for i in range(N):
    k = int(input()) + 1
    h = int(input()) - 1
    print(comb(k+h, h))
    
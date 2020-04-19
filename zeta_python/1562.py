N = int(input())
#3차원
temper = [0 for i in range(N)]
count = 0

def C(t):
    T = [0 for i in range(10)]
    for i in t:
        T[i] = True
    if all(T) is True:
        return True
    else:
        return False

def I(l, k):
    global count
    if 0 <= k <= 9:
        temper[l - 1] = k
        if l == 1:
            if C(temper):
                count += 1
                count %= 1000000000
        else:
            I(l - 1, k - 1)
            I(l - 1, k + 1)
    else:
        return

for i in range(1,10):
    I(N, i)
print(count % 1000000000)
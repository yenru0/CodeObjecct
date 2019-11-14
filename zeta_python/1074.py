Size, r, c = map(int, input().split())
count = -1

def Z(X: int, Y: int, size: int):
    global count, r, c
    if size == 0:
        count += 1
        if X == c and Y == r:
            print(count)
            exit()
    else:
        p = 2**(size-1)
        Z(X, Y, size-1)
        Z(X+p, Y, size-1)
        Z(X, Y+p, size-1)
        Z(X+p, Y+p, size-1)

Z(0,0,Size)
'''
Z(0,0,2)
    Z(0,0,1)
        Z(0,0,0)  each count() if size = 0
        Z(1,0,0)
        Z(0,1,0)
        Z(1,1,0)
    Z(2,0,1)
        Z(2,0,0)
        Z(3,0,0)
        Z(2,1,0)
        Z(3,1,0)
    Z(0,2,1)
    Z(2,2,1)
Z(1*2^2,0,2)
    Z(1*2^2, 0, 1)
        Z(2^2 + 1*2^0)
'''
T = int(input())
for i in range(T):
    x1, y1, r1, x2, y2, r2 = map(int, input().split())
    dist:int = (x1-x2)**2 + (y1- y2)**2
    distR:int = (r1+r2)**2
    if dist == 0 and r1 == r2:
        print(-1)
    elif dist < (r1-r2)**2:
        print(0)
    elif dist == (r1-r2)**2:
        print(1)
    elif dist > distR:
        print(0)
    elif dist < distR:
        print(2)
    elif dist == distR:
        print(1)

x1, y1, x2, y2 = map(int, input().split())
x3, y3, x4, y4 = map(int, input().split())

vx1, vy1 = x2 - x1, y2 - y1
vx2, vy2 = x4 - x3, y4 - y3
if vx1 == 0 and vy1 == 0:
    print(0)
elif vx1 == 0 or vy1 == 0:
    if vx1 == 0:
        rx = x1
        ry = (vy2/vx2) * (rx - x3) + y3

        t0 = (ry - y1) / vy1
        t1 = (rx - x3) / vy2
    else:
        rx = x3
        ry = (vy1 / vx1) * (rx-x1) + y1

        t0 = (rx - x1) / vx1
        t1 = (ry - y3) / vy2
    if t0 < 0 or t0 > 1 or t1 < 0 or t1 > 1: print(0)
    else: print(1)
else:
    a0 = vy1 / vx1
    a1 = vy2 / vx2

    rx = (a0 * x1 - a1 * x3 + y3 - y1 )/ (a0 - a1)
    ry = a0 * (rx - x1) + y1

    t0 = (rx - x1) / vx1
    t1 = (rx - x3) / vx2
    if a0 == a1: print(0)
    elif t0 < 0 or t0 > 1 or t1 < 0 or t1 > 1: print(0)
    else:
        print(1)
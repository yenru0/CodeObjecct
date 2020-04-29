I = list(map(int, input().split()))
while I[0] != 0:
    I.sort()
    if I[0] ** 2 + I[1] ** 2 == I[2] ** 2:
        print("right")
    else:
        print("wrong")


    I = list(map(int, input().split()))
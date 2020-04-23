N = int(input())
for i in range(N):
    for j in range(N):
        if j % 2 == 0:
            print("*", end='')
        else:
            print(" ", end='')
    print()
    for j in range(N):
        if j % 2 == 1:
            print("*", end='')
        else:
            print(" ", end='')
    print()

import sys
for _ in range(3):
    N = int(sys.stdin.readline())
    t = 0
    for i in range(N):
        t += int(sys.stdin.readline())
        
    if t > 0:
        print("+")
    elif t < 0:
        print("-")
    else:
        print("0")
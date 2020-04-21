N = int(input())
def move(f, t):
    print(f, t)

def hanoi(n, f, b, t):
    if n == 1:
        move(f, t)
    else:
        hanoi(n-1, f, t, b)
        move(f, t)
        hanoi(n-1, b, f, t)

print(2**N-1)
hanoi(N, 1,2,3)
N = int(input())

A = ([" "] * N + ["\n"]) * N

def B(n, x, y):
    global A
    if n == 0:
        A[(N+1)*y+x] = "*"
        return
    c = n//3
    for i in range(3):
        for j in range(3):
            if i == 1 and j == 1:
                continue
            B(c, x+c*i, y+c*j)

B(N, 0, 0)
for i in A:
    print(i, end="")

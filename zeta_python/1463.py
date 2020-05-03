M = {1:0, 2:1, 3:1, 10:3}
def ndp(n):
    global M
    stack = []
    if n in M:
        return M[n]
    if n % 3 == 0:
        stack.append((n, n//3))
    if n %2 == 0:
        stack.append((n, n//2))
    stack.append((n, n-1))
    while stack:
        node = stack[-1]
        if node[1] in M:
            if node[0] in M:
                if M[node[1]] + 1 < M[node[0]]:
                    M[node[0]] = M[node[1]] + 1
            else:
                M[node[0]] = M[node[1]] + 1
            stack.pop()
            continue
        if node[1] == 1:
            M[node[0]] = 1
            stack.pop()
            continue
        if node[1] % 3 == 0:
            stack.append((node[1], node[1]//3))
        if node[1] % 2 == 0:
            stack.append((node[1], node[1]//2))
        stack.append((node[1], node[1] - 1))
    return M[n]
print(ndp(int(input())))
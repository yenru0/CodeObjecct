def solve(N, P, a, b):
    a -= 1
    b -= 1
    D = []
    D.append((a, 0))
    visited = [2147483647] * N
    visited[a] = 0
    if b == a:
        return 0
    while D:
        now, cost = D.pop(0)

        if now == b:
            return cost

        from_now_on = now - P[now]
        # - dir
        while from_now_on >= 0:
            if visited[from_now_on] > cost + 1:
                visited[from_now_on] = cost + 1
                D.append((from_now_on, cost + 1))
            from_now_on -= P[now]
        from_now_on = now + P[now]
        # + dir
        while from_now_on < N:
            if visited[from_now_on] > cost + 1:
                visited[from_now_on] = cost + 1
                D.append((from_now_on, cost + 1))
            from_now_on += P[now]

    return -1


if __name__ == "__main__":
    N = int(input())
    P = list(map(int, input().split()))
    a, b = map(int, input().split())
    print(solve(N, P, a, b))

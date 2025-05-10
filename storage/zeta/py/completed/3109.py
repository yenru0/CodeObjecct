def get_maximum_upper(R, C, M, start):
    now_r = start
    now_c = 0
    D = [(now_r, now_c)]
    dir = [(-1, 1), (0, 1), (1, 1)]
    while D:
        now_r, now_c = D.pop()
        M[now_r][now_c] = 1
        insert = []
        for dr, dc in dir:
            new_r, new_c = now_r + dr, now_c + dc
            if new_c == C - 1:
                if 0 <= new_r < R:
                    M[new_r][new_c] = 1
                    return True
                continue

            if 0 <= new_r < R:
                if M[new_r][new_c]:
                    continue
                insert.append((new_r, new_c))
        if insert:
            D.extend(insert[::-1])
    return []


def solve(R, C, M):
    cnt = 0
    for i in range(R):
        path = get_maximum_upper(R, C, M, i)
        if path:
            cnt += 1
        else:
            continue

    return cnt


if __name__ == '__main__':
    import sys

    R, C = map(int, sys.stdin.readline().split())
    M = [list(map(lambda x: 1 if x == "x" else 0, sys.stdin.readline().rstrip())) for _ in range(R)]

    print(solve(R, C, M))

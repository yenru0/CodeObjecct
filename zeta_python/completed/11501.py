def solve(N, P):
    s = 0
    r, a = partition(P)
    s += a
    while (r):
        r, a = partition(r)
        s += a
    return s


def partition(part: list):
    max_p = max(part)

    m_idx = 0
    for i in range(len(part)):
        if part[i] == max_p:
            m_idx = i
    remain = part[m_idx + 1:]
    after = part[:m_idx]
    amount = sum([max_p - v for v in after])
    return remain, amount


if __name__ == '__main__':
    T = int(input())
    for _ in range(T):
        N = int(input())
        P = list(map(int, input().split()))
        print(solve(N, P))

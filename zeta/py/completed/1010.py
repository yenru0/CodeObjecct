def case(N, M):
    """
    정의역의 원소가 N개이고 치역의 공역의 원소가 M개인 일대일 '증가' 함수의 경우의 수
    (M)C(N)
    """
    ans = 1
    for i in range(M - N + 1, M + 1):
        ans *= i
    for i in range(1, N + 1):
        ans //= i
    return ans


def solve(T, I):
    return "\n".join([str(case(*I[i])) for i in range(T)])


if __name__ == '__main__':
    T = int(input())
    print(solve(T, [list(map(int, input().split())) for _ in range(T)]))

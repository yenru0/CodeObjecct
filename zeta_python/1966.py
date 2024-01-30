from collections import deque


def case(N, M, I):
    q_id = deque(range(N))
    q_priority = deque(I)

    q_m_p = q_priority[M]

    q_id


if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        print(case(*map(int, input().split()), list(map(int, input().split()))))

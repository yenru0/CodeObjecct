import sys
import heapq as hq

input = sys.stdin.readline

INF: int = 1 << 32
FULL: int


def vis_full(vis) -> int:
    return vis == FULL


def vis_add(vis, i) -> int:
    return vis | (1 << i)


def vis_is_in(vis, i) -> bool:
    return vis & (1 << i) != 0


def get_min_cost(N: int, W: list[list[int]]):
    start = 0
    st_vis = 0
    st_vis = vis_add(st_vis, 0)

    M = [[-1 for _ in range(FULL + 1)] for _ in range(N)]
    stack = [(0, st_vis)]

    while stack:
        now, vis = stack[-1]
        if vis_full(vis):
            stack.pop()
            if W[now][start] != 0:
                M[now][vis] = W[now][start]
            else:
                M[now][vis] = INF
            continue

        if M[now][vis] == -1:
            M[now][vis] += 1
            for i in range(N):
                if (not vis_is_in(vis, i)) and W[now][i] != 0:
                    if M[i][vis_add(vis, i)] <= 0:
                        stack.append((i, vis_add(vis, i)))

        elif M[now][vis] == 0:
            stack.pop()
            s = []
            for i in range(N):
                if not vis_is_in(vis, i) and W[now][i] != 0:
                    s.append(M[i][vis_add(vis, i)] + W[now][i])
            M[now][vis] = min(s) if s else INF
        else:
            continue
    return M[0][st_vis]


if __name__ == "__main__":
    N = int(input())
    W = [list(map(int, input().split())) for _ in range(N)]
    FULL = (1 << N) - 1
    print(get_min_cost(N, W))

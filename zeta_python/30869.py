import sys
import heapq

input = sys.stdin.readline

if __name__ == "__main__":
    N, M, K = map(int, input().split())
    L = [[] for _ in range(M + 1)]
    for _ in range(M):
        s, e, t, g = map(int, input().split())
        L[s].append((e, t, g))
    res = 100000000000

    D = []
    vis_tmp = [0] * (N + 1)
    vis_tmp[1] = 1
    heapq.heappush(
        D,
        (0, 1, K, vis_tmp),
    )
    while D:
        now_t, now_stop, now_k, vis = heapq.heappop(D)
        vis[now_stop] = 1
        if now_stop == N:
            res = min(res, now_t)
            break

        for e, t, g in L[now_stop]:
            if vis_tmp[e]:
                continue
            if now_t % g == 0:
                heapq.heappush(D, (now_t + t, e, now_k, vis.copy()))
            else:
                heapq.heappush(D, (now_t + -now_t % g + g + t, e, now_k, vis.copy()))
                # 버스 소환술 사용
                if now_k >= 1:
                    heapq.heappush(D, (now_t + t, e, now_k - 1, vis.copy()))
    else:
        res = -1
    print(res)

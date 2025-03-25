import sys

input = sys.stdin.readline


class TriangleTrees:
    def __init__(self, n, m, e):
        """
        input graph should be triangle-trees
        """
        self.n, self.m = n, m
        self.e: list[int] = e
        self.e_find: list[list[int]] = [[] for _ in range(self.n)]
        for u, v in self.e:
            self.e_find[u - 1].append(v - 1)
            self.e_find[v - 1].append(u - 1)

    def check_cycle(self) -> bool:
        D = [(-1, i) for i in range(self.n)]
        vis = [0] * self.n
        cycle_flag = False
        while D:
            before, now = D.pop()
            if vis[now]:
                continue
            vis[now] = 1
            for nxt in self.e_find[now]:
                if nxt == before:
                    continue
                elif vis[nxt]:
                    cycle_flag = True
                    break
                else:
                    D.append((now, nxt))
            if cycle_flag:
                break
        return cycle_flag

    def solve(self) -> list[int]:
        K_MAX = 3 if self.check_cycle() else 2
        vis = [0] * self.n
        colour = [-1] * self.n

        D = [(i, i) for i in range(self.n)]
        while D:
            (before, now) = D.pop()
            if vis[now]:
                continue
            else:
                vis[now] = 1

            now_color = (colour[before] + 1) % K_MAX
            colour[now] = now_color

            for nxt in self.e_find[now]:
                if not vis[nxt]:
                    D.append((now, nxt))
        return colour


if __name__ == "__main__":
    n, m = map(int, input().split())
    e = [tuple(map(int, input().split())) for _ in range(m)]
    print(*map(lambda x: x + 1, TriangleTrees(n, m, e).solve()), sep=" ")

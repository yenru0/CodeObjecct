import heapq


class DietPlan:
    def __init__(self, n, m, k, p):
        self.n: int = n
        self.m: int = m
        self.k: int = k
        self.p: list[int] = p

    def solve(self) -> int:
        now_remain = self.m

        D: list = []
        heapq.heapify(D)

        final_day = 0
        is_consume = False
        is_finish = False
        for i, p in enumerate(self.p):
            is_consume = False
            while now_remain - p < 0:
                if self.k > 0:
                    # 쿠키 먹기
                    if D:
                        tmp1 = -heapq.heappop(D)
                        tmp2 = p
                        if tmp1 <= tmp2:
                            p = 0
                            is_consume = True
                            heapq.heappush(D, -tmp1)
                        else:
                            now_remain += tmp1
                    else:
                        p = 0
                        is_consume = True
                    self.k -= 1

                else:
                    final_day = i
                    is_finish = True
                    break
            else:
                if not is_consume:
                    now_remain -= p
                    heapq.heappush(D, -p)
            if is_finish:
                break
        else:
            final_day = self.n

        return final_day


if __name__ == "__main__":
    n, m, k = map(int, input().split())
    p = list(map(int, input().split()))
    print(DietPlan(n, m, k, p).solve())

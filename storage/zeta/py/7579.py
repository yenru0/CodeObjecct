import sys

input = sys.stdin.readline

class ApplicationMemorySwap:
    def __init__(self, n: int, m: int, mem: list[int], cost: list[int]):
        self.n: int = n
        self.m: int = m
        self.mem: list[int] = mem
        self.cost: list[int] = cost
        
    def solve(self) -> int:
        knapsack = [0] * (self.m + 1)


if __name__ == '__main__':
    n , m = map(int, input().split())
    mem = list(map(int, input().split()))
    cost = list(map(int, input().split()))
    solver = ApplicationMemorySwap(n, m, mem, cost)
    print(solver.solve())

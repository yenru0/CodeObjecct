import sys

input = sys.stdin.readline


class PrefixSum:
    def __init__(self, N: int, arr: list[int]):
        self.N = N
        self.__arr = arr
        self.__cumulative = [0]
        for i in range(N):
            self.__cumulative.append(self.__cumulative[-1] + self.__arr[i])
    
    def sum_range(self, left: int, right: int) -> int:
        return self.__cumulative[right] - self.__cumulative[left]

    def solve(self, S: int) -> int:
        mindist = 10000001
        left = 0
        right = 0
        while right <= N:
            if left > right:
                right += 1
                continue
            if self.sum_range(left, right) >= S:
                mindist = min([right - left, mindist])
                left += 1
            else:
                right += 1
        if mindist > self.N:
            return 0
        elif mindist == 0:
            return 1
        return mindist
        


if __name__ == "__main__":
    N, S = map(int, input().split())
    solver = PrefixSum(N, list(map(int, input().split())))
    
    print(solver.solve(S))

import sys

input = sys.stdin.readline


class ReverseSortTracer:
    def __init__(self, n, arr):
        self.n = n
        self.arr = arr

    def sort_trace(self) -> list[tuple[int, int]]:
        qx = []
        sorted_arr = self.arr.copy()

        for i in range(1, self.n):
            l = i
            r = sorted_arr.index(i) + 1
            for ptr in range((r - l + 1) // 2):
                sorted_arr[l + ptr - 1], sorted_arr[r - 1 - ptr] = \
                    sorted_arr[r - 1 - ptr], sorted_arr[l + ptr - 1]
            qx.append((l, r))

        return qx


if __name__ == "__main__":
    n = int(input())
    arr = list(map(int, input().split()))

    solved = ReverseSortTracer(n, arr).sort_trace()
    print(len(solved))
    for pair in solved:
        print(*pair)

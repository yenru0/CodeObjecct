class RemoteController:
    """
    start at number 100
    action:
        press number not broken
        press ++
        press --

    example(5457 with broken 6 7 8):
        5 4 5 5 + +
    """

    def __init__(self, broken: list[int]):
        self.broken = broken

    def solve(self, target: int) -> int:
        """return: number of press buttons to reach target number"""
        mins = abs(target - 100)
        for i in range(0, 1000000 + 1):
            nums = list(map(int, str(i)))
            if all(False if s in self.broken else True for s in nums):
                mins = min(abs(target - i) + len(nums), mins)

        return mins


if __name__ == "__main__":
    N = int(input())
    M = int(input())
    broken = list(map(int, input().split())) if M != 0 else []
    solver = RemoteController(broken)
    print(solver.solve(N))

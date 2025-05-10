import sys

input = sys.stdin.readline

class Pane2048:
    def __init__(self, N: int, pane: list[list[int]]):
        self.N = N
        self.pane = pane
        
    @staticmethod
    def move(direction: int, N: int, pane: list[list[int]]):
        pass

if __name__ == '__main__':
    pass
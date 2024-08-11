import sys
from dataclasses import dataclass

input = sys.stdin.readline

@dataclass
class interval:
    start: int
    end: int


if __name__ == "__main__":
    N = int(input())
    org = [interval(*map(int, input().split())) for _ in range(N)]
    print(max([max(org, key=lambda x: x.start).start - min(org, key=lambda x: x.end).end, 0]))

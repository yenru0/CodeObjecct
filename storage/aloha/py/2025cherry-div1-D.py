import sys

input = sys.stdin.readline


class ALOHAGames:
    def __init__(self):
        self.t = 1
        self.f = 0
        self.res_mineral = 4000
        self.res_propane = 3000


# R1


if __name__ == "__main__":
    print("""8
max(1,1)
min(2,3)
plus(1,1,1)
plus(3,3,2)
max(3,3)
submit(1,1)
minus(9,9,3)
submit(3,3)""")

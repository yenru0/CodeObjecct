import sys
import itertools as it

input = sys.stdin.readline

if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        N = int(input())
        bob = list(map(str, input().strip()))
        alice = list(map(str, input().strip()))
        bob_min = min([int("".join(bob)), int("".join(bob[::-1]))])
        alice_all = sum(
            [
                [int("".join(p)) for p in it.permutations(alice, r)]
                for r in range(1, len(alice) + 1)
            ],
            start=[],
        )

        print(max(filter(lambda x: x < bob_min, alice_all)))

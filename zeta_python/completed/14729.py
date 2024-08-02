import sys
input = sys.stdin.readline
[print(f"{x:.3f}")for x in list(sorted(float(input())for _ in range(int(input()))))[:7]]
import sys
input = sys.stdin.readline

if __name__ == '__main__':
    T = int(input())
    for _ in range(T):
        input()
        count = 0
        R, C = map(int, input().split())
        M = [list(input()) for _ in range(R)]

        for r in range(R):
            for c in range(C):
                if c < C - 2 and M[r][c] == '>' and M[r][c+1] == 'o' and M[r][c+2] == '<':
                    count += 1
                elif r < R - 2 and M[r][c] == 'v' and M[r+1][c] == 'o' and M[r+2][c] == '^':
                    count += 1
        print(count)
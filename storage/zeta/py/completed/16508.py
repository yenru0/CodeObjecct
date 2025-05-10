import sys

input = sys.stdin.readline

INF_MAX = 1000000 * 50


def transpose(word: str):
    dictionary = [0] * 26
    for c in word:
        dictionary[ord(c) - 65] += 1
    return dictionary


def check(T: list, word: list):
    return all([word[i] >= T[i] for i in range(26)])


def merge(a: list, b: list):
    return [a[i] + b[i] for i in range(26)]


if __name__ == "__main__":
    T = transpose(input().rstrip())
    N = int(input())
    P = [
        (int(c), transpose(w)) for c, w in (input().rstrip().split() for _ in range(N))
    ]
    D = []
    min_price = INF_MAX

    D.append((0, 0, [0] * 26))
    while D:
        index, cost, word = D.pop()
        word: list
        if index >= N:
            if check(T, word):
                min_price = min(min_price, cost)
            continue
        else:
            D.append((index + 1, cost + P[index][0], merge(word, P[index][1])))
            D.append((index + 1, cost, word.copy()))
    print(min_price) if min_price < INF_MAX else print(-1)

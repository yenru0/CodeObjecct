from collections import deque


def solve(N):
    queue = deque(range(1, N + 1))
    cnt = 1
    ret = []
    while len(queue) != 1:
        if cnt % 2 == 0:
            queue.append(queue.popleft())
        else:
            ret.append(queue.popleft())
        cnt += 1
        cnt %= 2

    ret.append(queue[0])
    return " ".join(map(str, ret))


if __name__ == "__main__":
    print(solve(int(input())))

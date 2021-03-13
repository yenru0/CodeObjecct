from collections import deque


def solve(N):
    queue = deque(range(1, N + 1))
    cnt = 1
    while len(queue) != 1:
        if cnt % 2 == 0:
            queue.append(queue.popleft())
        else:
            queue.popleft()
        cnt += 1
        cnt %= 2

    return queue.pop()


if __name__ == "__main__":
    print(solve(int(input())))

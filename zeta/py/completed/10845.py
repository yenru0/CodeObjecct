import sys
from collections import deque

queue = deque()


def dispatch(command: str, *args):
    if command == "push":
        queue.append(int(args[0]))
    elif command == "pop":
        if queue:
            return queue.popleft()
        else:
            return -1
    elif command == "size":
        return len(queue)
    elif command == "empty":
        return 0 if queue else 1
    elif command == "front":
        if queue:
            return queue[0]
        else:
            return -1
    elif command == "back":
        if queue:
            return queue[-1]
        else:
            return -1


if __name__ == "__main__":
    N = int(sys.stdin.readline())
    for _ in range(N):
        d = dispatch(*sys.stdin.readline().strip().split())
        if d is not None:
            print(d)

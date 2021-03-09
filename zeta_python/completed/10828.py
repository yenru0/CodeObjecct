import sys


class Stack(list):
    def push(self, *args, **kwargs):
        self.append(args[0])

    def size(self, *args, **kwargs):
        return self.__len__()

    def empty(self, *args, **kwargs):
        return 1 if self.__len__() == 0 else 0

    def top(self, *args, **kwargs):
        return self.__getitem__(-1)

    def dispatch(self, exp):
        t = exp.strip().split()
        try:
            return self.__getattribute__(t[0])(*map(int, t[1:]))
        except:
            return -1


if __name__ == '__main__':
    s = Stack()
    N = int(sys.stdin.readline())
    for i in range(N):
        exp = sys.stdin.readline()
        ret = s.dispatch(exp)
        if ret is not None:
            print(ret)

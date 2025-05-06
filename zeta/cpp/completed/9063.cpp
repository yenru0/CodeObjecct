#include <iostream>
#include <algorithm>

using namespace std;

int main() {
    int N;
    cin >> N;
    int xs[N], ys[N];
    int x, y;
    for (int i = 0; i < N; i++) {
        cin >> x >> y;
        xs[i] = x;
        ys[i] = y;
    }
    sort(xs, xs + N);
    sort(ys, ys + N);

    cout << (xs[N - 1] - xs[0]) * (ys[N - 1] - ys[0]) << endl;
}
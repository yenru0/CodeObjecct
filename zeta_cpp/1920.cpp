#include <iostream>
#include <algorithm>

using namespace std;

bool isExist(int x, int L[], int size) { // binsearch
    int begin = 0;
    int end = size - 1;
    int mid;
    int sep;
    while (begin <= end) {
        mid = (begin + end) / 2;
        sep = L[mid];
        if (x == sep) {
            return true;
        } else if (x < sep) {
            end = mid - 1;
        } else {
            begin = mid + 1;
        }
    }
    return false;
}

int main() {
    int N, M;
    ios::sync_with_stdio(0);
    cin.tie(NULL);
    cout.tie(NULL);
    cin >> N;
    int A[N];
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }
    sort(A, A + N);

    cin >> M;
    int x;
    for (int i = 0; i < M; i++) {
        cin >> x;
        if (isExist(x, A, N)) {
            cout << "1\n";
        } else {
            cout << "0\n";
        }

    }
    return 0;
}
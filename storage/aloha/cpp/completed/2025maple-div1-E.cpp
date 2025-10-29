#define N 8
#include <iostream>
#include <queue>
#include <utility>

using namespace std;
using pii = pair<int, int>;

int n = N, arr[N];
queue<pii> q;

void sweet(int l, int r);

int main() {
    sweet(0, N-1);
    for (int e : arr) cout << e << " ";
    cout << "\n";
}

void sweet(int l, int r) {
    q.push({l, r});

    while (!q.empty()) {
        pii now = q.front(); q.pop();

        int l = now.first, r = now.second;
        int mid = (l+r)/2;

        if (l > r) continue;
        arr[mid] = n--;

        if (mid-1 - l > r - (mid+1)) {
            q.push({l, mid-1});
            q.push({mid+1, r});
        } else {
            q.push({mid+1, r});
            q.push({l, mid-1});
        }
    }
}
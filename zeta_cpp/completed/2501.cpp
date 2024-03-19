#include <iostream>
#include <cmath>

using namespace std;

int main() {
    int count = 1;
    int N, K;

    cin >> N >> K;
    if(K == 1) {
        cout << 1 << endl;
        return 0;
    }
    for(int i=2;i<=N;i++) {
        if (N%i == 0) {
            count += 1;
            if (count == K) {
                cout << i << endl;
                break;
            }
        }
    }
    if (K > count) {
        cout << 0 << endl;
    }

    return 0;
}
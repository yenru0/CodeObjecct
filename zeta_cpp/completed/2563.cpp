#include <iostream>

using namespace std;

void add_paper(int P[], int x, int y) {
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
            P[(x - 1 + i) * 100 + y - 1 + j] = 1;
        }
    }
}


int main() {
    int Paper[100 * 100] = {0,};
    int N;

    cin >> N;
    int _x, _y;
    for (int i = 0; i < N; i++) {
        cin >> _x >> _y;
        add_paper(Paper, _x, _y);
    }
    int A = 0;
    for (int i = 0; i < 10000; i++) {
        A += Paper[i];
    }
    cout << A;
    return 0;
}
#include <iostream>

using namespace std;

int main() {
    int n, inv, curr, before;
    int delta;

    cin >> n >> before;

    bool bflag = false;
    for (int i = 1; i <= n; i++) {
        if (bflag) {
            cout << i - 1 << " " << i << endl;
        } else {
            cout << i << " " << i << endl;
        }
        cin >> curr;
        if(curr == 0) {
            break;
        }
        
        delta = curr - before;
        if(delta > 0) {
            bflag = true;
        } else {
            bflag = false;
            before = curr;
        }
    }

    return 0;
}
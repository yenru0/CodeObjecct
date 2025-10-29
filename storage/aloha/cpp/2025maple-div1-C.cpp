#define MAXN 50
#include <iostream>

using namespace std;

char ansLine;
int N, T, A[MAXN][MAXN], B[MAXN][MAXN], ansNum;

void check();

int main() {
    ios_base::sync_with_stdio(0);
    cin.tie(0); cout.tie(0);
    
    while (!cin.eof()) {
        cin >> T;
        
        if (T == 1) {
            cin >> N;
            for (int i = 0; i < N; i++)
                for (int j = 0; j < N; j++)
                    cin >> A[i][j];
            for (int i = 0; i < N; i++) cout << "H 1\n";
            
            if (N % 2)
                for (int i = 0; i < N; i++) A[0][i] = !A[0][i];

        } else if (T == 2) {
            cin >> N;
            for (int i = 0; i < N; i++)
                for (int j = 0; j < N; j++)
                    cin >> B[i][j];
            
            check();
            if (ansNum == -1) cout << 0 << "\n";
            else cout << 1 << "\n" << ansLine << " " << ansNum << "\n";
        }

    }
}

void check() {
    int row = -1, col = -1;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            if (A[i][j] != B[i][j]) {
                if (row == i) ansLine = 'H';
                else if (col == j) ansLine = 'V';

                row = i, col = j;
            }
        }
    }

    if (ansLine == 'H') ansNum = row+1;
    else if (ansLine == 'V') ansNum = col+1;
    else ansNum = -1;
}
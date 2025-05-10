#include<iostream>
#include<vector>

using namespace std;

auto sum(int N, int M, int A[], int B[], int S[]) {
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            S[i * M + j] = A[i * M + j] + B[i * M + j];
        }
    }
    return S;
}

void display(int N, int M, int S[]) {
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            cout << S[i * M + j];
            if (j < M - 1) {
                cout << " ";
            }
        }
        cout << endl;
    }
}

int main() {
    int N, M;
    cin >> N >> M;
    int A[N * M];
    int B[N * M];
    int S[N * M];
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            cin >> A[i * M + j];
        }
    }
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            cin >> B[i * M + j];
        }
    }
    sum(N, M, A, B, S);
    display(N, M, S);
    return 0;
}
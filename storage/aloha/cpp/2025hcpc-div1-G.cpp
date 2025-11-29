#include <algorithm>
#include <iostream>
#include <queue>
#include <stdint.h>
#include <stdlib.h>

using namespace std;

typedef struct Node {
    int value;
    struct Node *left;
    struct Node *right;
    bool status;
} Node;

int main() {
    cin.tie(NULL);
    ios_base::sync_with_stdio(0);
    
    auto data = new vector<int>;
    auto pq = new vector<int>;

    uint64_t n;
    int temp;
    cin >> n;
    cin >> temp;
    pq->push_back(temp);
    cout << 1 << "\n";

    Node * node = NULL;
    for (size_t i = 1; i < n; i ++) {
        cin >> temp;
        pq->push_back(temp);
    }


    if (node == NULL) {
        cout << 1 << "\n";
    }
    
    delete pq;
    delete data;
}
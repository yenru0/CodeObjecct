#include <algorithm>
#include <cstdint>
#include <iostream>
#include <string>
#define let auto
#define fn auto
using namespace std;

typedef size_t usize;
typedef uint64_t u64;

const let WHITE_KING = "Whiteking";
const let BLACK_KING = "Blackking";

fn fastio() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
}

fn main() -> int {
    usize n;// count of logs
    cin >> n;

    let arr = new u64[n];// length of each log
    for (usize i = 0; i < n; i++) {
        cin >> arr[i];
    }

    string who;
    cin >> who;

    let first = (who[0] == 'W') ? false : true;

    let g = 0;
    for (usize i = 0; i < n; i++) {
        let e = arr[i] - 2;
        g ^= e;
    }
    
    //
    // W0B WOOOB

    if (g == 0) {
        cout << (first ? WHITE_KING : BLACK_KING);
    } else {
        cout << (!first ? WHITE_KING : BLACK_KING);
    }

    // free
    delete[] arr;
    return 0;
}
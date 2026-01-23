#include <iostream>
#include <stdint.h>
#include <vector>
#define let auto

typedef int i32;
typedef size_t usize;
typedef long long i64;
typedef uint64_t u64;

using namespace std;

let fastio() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
}

enum Turn {
    A,
    B,
};

enum Winnable {
    UNDEF,
    WIN,
    LOSE,
};

typedef struct GS {
    u64 b1, b2, b3;
    Winnable **dp;
} GameState;

let dfs(GameState *state, Turn turn, u64 k1, u64 k2) -> Winnable {
    let opposite = turn == A ? B : A;
    if (k2 > k1) {
        let temp = k2;
        k2 = k1;
        k1 = temp;
    }
    if (state->dp[k1][k2] != UNDEF) {
        return state->dp[k1][k2];
    } else {
        let b1 = state->b1;
        let b2 = state->b2;
        let b3 = state->b3;

        let sigs = vector<Winnable>();
        if (k1 >= b1) {
            sigs.push_back(dfs(state, opposite, k1 - b1, k2));
        }
        if (k1 >= b2) {
            sigs.push_back(dfs(state, opposite, k1 - b2, k2));
        }
        if (k1 >= b3) {
            sigs.push_back(dfs(state, opposite, k1 - b3, k2));
        }
        if (k2 >= b1) {
            sigs.push_back(dfs(state, opposite, k1, k2 - b1));
        }
        if (k2 >= b2) {
            sigs.push_back(dfs(state, opposite, k1, k2 - b2));
        }
        if (k2 >= b3) {
            sigs.push_back(dfs(state, opposite, k1, k2 - b3));
        }

        let flag = LOSE;
        for (let sig: sigs) {
            if (sig == LOSE) {
                flag = WIN;
            }
        }
        state->dp[k1][k2] = flag;
        return flag;
    }
}

const u64 MAX_K = 502;

let main()
        -> i32 {
    fastio();

    let b1 = (u64) 0;
    let b2 = (u64) 0;
    let b3 = (u64) 0;
    cin >> b1 >> b2 >> b3;
    let dp = (Winnable **) calloc(MAX_K, sizeof(Winnable *));

    {// init
        for (u64 i = 0; i < MAX_K; i++) {
            dp[i] = (Winnable *) calloc(MAX_K, sizeof(Winnable));
        }

        for (u64 i = 0; i < b1; i++) {
            for (u64 j = 0; j < b1; j++) {
                dp[i][j] = LOSE;
            }
        }
        for (u64 i = 0; i < b1; i++) {
            dp[i][b1] = WIN;
            dp[b1][i] = WIN;
        }
        dp[b1][b1] = LOSE;
    }

    let gs = GameState{
            .b1 = b1,
            .b2 = b2,
            .b3 = b3,
            .dp = dp};

    for (let i = (usize) 0; i < 5; i++) {
        u64 k1, k2;
        cin >> k1 >> k2;

        let sig = dfs(&gs, A, k1, k2);

        if (sig == WIN) {
            cout << "A" << endl;
        } else {
            cout << "B" << endl;
        }
    }

    {// delete
        for (let i = (u64) 0; i < MAX_K; i++) {
            free(dp[i]);
        }
        free(dp);
    }
}
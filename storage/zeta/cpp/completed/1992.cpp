#include <cstdlib>
#include <iostream>
#include <tuple>
#include <vector>

#define let auto
#define fn auto
#define usize size_t
using namespace std;

fn fastio() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
}

fn check_all(usize a, usize b, usize c, usize d) -> bool {
    return a == b && b == c && c == d;
}

struct Node {
    bool is_fb;
    usize repr;
    Node *ul;
    Node *ur;
    Node *dl;
    Node *dr;
};

fn print_node(Node *node) -> void {
    if (node->is_fb) {
        cout << node->repr;
    } else {
        cout << "(";
        print_node(node->ul);
        print_node(node->ur);
        print_node(node->dl);
        print_node(node->dr);
        cout << ")";
    }
}

fn free_node(Node *node) -> void {
    if (node->is_fb) {

    } else {
        free_node(node->ul);
        free_node(node->ur);
        free_node(node->dl);
        free_node(node->dr);
    }
    free(node);
}

fn quadtree(usize *arr, usize size, usize n, usize lo) -> Node * {
    if (n <= 2) {
        let a = arr[lo];
        let b = arr[lo + 1];
        let c = arr[lo + size];
        let d = arr[lo + size + 1];
        let clk = check_all(a, b, c, d);

        Node *node = (Node *) malloc(sizeof(Node));
        if (clk) {
            node->is_fb = true;
            node->repr = a;
        } else {
            node->is_fb = false;
            Node *tmp1 = (Node *) malloc(sizeof(Node));
            tmp1->is_fb = true;
            tmp1->repr = a;
            node->ul = tmp1;
            Node *tmp2 = (Node *) malloc(sizeof(Node));
            tmp2->is_fb = true;
            tmp2->repr = b;
            node->ur = tmp2;
            Node *tmp3 = (Node *) malloc(sizeof(Node));
            tmp3->is_fb = true;
            tmp3->repr = c;
            node->dl = tmp3;
            Node *tmp4 = (Node *) malloc(sizeof(Node));
            tmp4->is_fb = true;
            tmp4->repr = d;
            node->dr = tmp4;
        }
        return node;
    } else {
        let lo_up_left = lo;
        let lo_up_right = lo + n / 2;
        let lo_down_left = lo + n / 2 * size;
        let lo_down_right = lo + n / 2 * size + n / 2;

        let q_up_left = quadtree(arr, size, n / 2, lo_up_left);
        let q_up_right = quadtree(arr, size, n / 2, lo_up_right);
        let q_down_left = quadtree(arr, size, n / 2, lo_down_left);
        let q_down_right = quadtree(arr, size, n / 2, lo_down_right);

        let fullbank_up_left = (q_up_left)->is_fb;
        let fullbank_up_right = (q_up_right)->is_fb;
        let fullbank_down_left = (q_down_left)->is_fb;
        let fullbank_down_right = (q_down_right)->is_fb;

        Node *node = (Node *) malloc(sizeof(Node));
        if (fullbank_up_left && fullbank_up_left == fullbank_up_right && fullbank_up_right == fullbank_down_left && fullbank_down_left == fullbank_down_right && q_up_left->repr == q_up_right->repr && q_up_right->repr == q_down_left->repr && q_down_left->repr == q_down_right->repr) {
            let repr = q_up_left->repr;
            free_node(q_up_left);
            free_node(q_up_right);
            free_node(q_down_left);
            free_node(q_down_right);

            node->is_fb = true;
            node->repr = repr;
        } else {
            node->ul = q_up_left;
            node->ur = q_up_right;
            node->dl = q_down_left;
            node->dr = q_down_right;
            node->is_fb = false;
        }
        return node;
    }
}

fn main() -> int {
    fastio();
    usize n;
    cin >> n;
    if (n == 1) {
        let tmp = (usize) 0;
        cin >> tmp;
        cout << tmp;
        return 0;
    }
    let arr = new usize[n * n];
    for (let i = (usize) 0; i < n * n; i++) {
        char ch;
        cin.get(ch);
        while (ch == '\n') {
            cin.get(ch);
        }
        arr[i] = ch - '0';
    }

    Node *node = quadtree(arr, n, n, 0);
    delete[] arr;
    print_node(node);
    free_node(node);
    return 0;
}
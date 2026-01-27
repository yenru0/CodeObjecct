#include <iostream>
#include <vector>

using namespace std;

#define let auto

void fastio() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
}

auto transform_to_postorder(vector<char> preorder, vector<char> inorder, vector<char> &postorder) -> bool {
    if (preorder.size() == 0) {
        return true;
    }

    let root = preorder[0];

    if (preorder.size() == 1) {
        if (preorder[0] == inorder[0]) {
            postorder[0] = root;
            return true;
        } else {
            return false;
        }
    }

    let find = inorder.begin();

    for (find = inorder.begin(); find != inorder.end(); find++) {
        if (*find == root) {
            break;
        }
    }

    if (find == inorder.end()) {
        return false;
    }

    let left_subtree_inorder = vector<char>(inorder.begin(), find);
    let right_subtree_inorder = vector<char>(find + 1, inorder.end());

    let left_size = left_subtree_inorder.size();
    let right_size = right_subtree_inorder.size();

    let left_subtree_preorder = vector<char>(preorder.begin() + 1, preorder.begin() + 1 + left_size);
    let right_subtree_preorder = vector<char>(preorder.begin() + 1 + left_size, preorder.end());

    // propagate
    let left_sub_post = vector<char>(left_subtree_inorder.size());
    let right_sub_post = vector<char>(right_subtree_inorder.size());
    let sig_left = transform_to_postorder(
            left_subtree_preorder, left_subtree_inorder, left_sub_post);
    let sig_right = transform_to_postorder(
            right_subtree_preorder, right_subtree_inorder, right_sub_post);

    if (sig_left == false || sig_right == false) {
        return false;
    }

    std::copy(
            left_sub_post.begin(), left_sub_post.end(), postorder.begin());
    std::copy(
            right_sub_post.begin(), right_sub_post.end(), postorder.begin() + left_sub_post.size());

    postorder[postorder.size() - 1] = root;
    return true;
}

auto main() -> int {
    fastio();

    string pre_str, in_str;

    while (cin >> pre_str && pre_str != "#") {
        cin >> in_str;

        vector<char> preorders(pre_str.begin(), pre_str.end());
        vector<char> inorders(in_str.begin(), in_str.end());

        vector<char> postorders(preorders.size());

        if (transform_to_postorder(preorders, inorders, postorders)) {
            for (let c: postorders) {
                cout << c;
            }
            cout << "\n";
        } else {
            cout << "Invalid tree\n";
        }
    }

    return 0;
}
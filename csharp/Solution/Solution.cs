// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode InsertIntoBST(TreeNode root, int val)
    {
        if (root is null) { return new(val); }
        if (root.val < val) { root.right = InsertIntoBST(root.right, val); }
        if (root.val > val) { root.left = InsertIntoBST(root.left, val); }
        return root;
    }
}

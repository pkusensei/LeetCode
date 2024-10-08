// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode TrimBST(TreeNode root, int low, int high)
    {
        if (root is null) { return null; }
        if (root.val < low) { return TrimBST(root.right, low, high); }
        if (high < root.val) { return TrimBST(root.left, low, high); }
        root.left = TrimBST(root.left, low, high);
        root.right = TrimBST(root.right, low, high);
        return root;
    }
}

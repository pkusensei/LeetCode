using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode TrimBST(TreeNode root, int low, int high)
    {
        if (root is null) { return null; }
        if (root.val < low) { return TrimBST(root.right, low, high); }
        if (root.val > high) { return TrimBST(root.left, low, high); }
        root.left = TrimBST(root.left, low, high);
        root.right = TrimBST(root.right, low, high);
        return root;
    }
}
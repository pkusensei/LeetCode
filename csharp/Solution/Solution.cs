using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int RangeSumBST(TreeNode root, int low, int high)
    {
        if (root is null) { return 0; }
        if (root.val < low) { return RangeSumBST(root.right, low, high); }
        else if (high < root.val) { return RangeSumBST(root.left, low, high); }
        else { return root.val + RangeSumBST(root.left, low, high) + RangeSumBST(root.right, low, high); }
    }
}
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int RangeSumBST(TreeNode root, int low, int high)
    {
        if (root is null) { return 0; }
        var res = low <= root.val && root.val <= high ? root.val : 0;
        return res + RangeSumBST(root.left, low, high) + RangeSumBST(root.right, low, high);
    }
}

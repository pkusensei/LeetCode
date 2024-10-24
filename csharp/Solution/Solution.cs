using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool FlipEquiv(TreeNode root1, TreeNode root2)
    {
        if (root1 is null && root2 is null) { return true; }
        if (root1 is null ^ root2 is null) { return false; }
        if (root1.val == root2.val)
        {
            return (FlipEquiv(root1.left, root2.right) && FlipEquiv(root1.right, root2.left))
                || (FlipEquiv(root1.left, root2.left) && FlipEquiv(root1.right, root2.right));
        }
        return false;
    }
}

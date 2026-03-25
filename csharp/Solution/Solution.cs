using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool FlipEquiv(TreeNode root1, TreeNode root2)
    {
        if (root1 is null) { return root2 is null; }
        if (root2 is null) { return false; }
        return root1.val == root2.val
            && ((FlipEquiv(root1.left, root2.right) && FlipEquiv(root1.right, root2.left))
                || (FlipEquiv(root1.left, root2.left) && FlipEquiv(root1.right, root2.right)));
    }
}

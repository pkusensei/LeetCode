using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode PruneTree(TreeNode root)
    {
        if (root is null) { return null; }
        root.left = PruneTree(root.left);
        root.right = PruneTree(root.right);
        if (root.left is null & root.right is null && root.val == 0) { return null; }
        return root;
    }
}

using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode PruneTree(TreeNode root) {
        if (root is null){ return null; }
        var left = PruneTree(root.left);
        var right = PruneTree(root.right);
        if (left is null && right is null && root.val==0){ return null; }
        (root.left, root.right) = (left, right);
        return root;
    }
}

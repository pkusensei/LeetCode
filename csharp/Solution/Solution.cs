using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxAncestorDiff(TreeNode root)
    {
        if (root is null) { return 0; }
        return Dfs(root, root.val, root.val);
    }

    static int Dfs(TreeNode node, int min, int max)
    {
        if (node is null) { return max - min; }
        min = Math.Min(min, node.val);
        max = Math.Max(max, node.val);
        return Math.Max(Math.Max(max - min, Dfs(node.left, min, max)), Dfs(node.right, min, max));
    }
}

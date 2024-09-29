using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int DiameterOfBinaryTree(TreeNode root)
    {
        return Dfs(root, 0).res;
    }

    (int res, int depth) Dfs(TreeNode node, int res)
    {
        if (node is null) { return (res, 0); }
        var (l_res, l_depth) = Dfs(node.left, res);
        var (r_res, r_depth) = Dfs(node.right, res);
        res = Math.Max(Math.Max(l_res, r_res), l_depth + r_depth);
        return (res, 1 + Math.Max(l_depth, r_depth));
    }
}
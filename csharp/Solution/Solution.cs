// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int LongestUnivaluePath(TreeNode root)
    {
        if (root is null) { return 0; }
        var a = Dfs(root.left, root.val) + Dfs(root.right, root.val);
        var b = LongestUnivaluePath(root.left);
        var c = LongestUnivaluePath(root.right);
        return Math.Max(a, Math.Max(b, c));
    }

    static int Dfs(TreeNode node, int num)
    {
        if (node is null) { return 0; }
        if (num == node.val)
        {
            return 1 + Math.Max(Dfs(node.left, num), Dfs(node.right, num));
        }
        return 0;
    }
}

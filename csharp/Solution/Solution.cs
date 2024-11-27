using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode SufficientSubset(TreeNode root, int limit)
    {
        var n = Dfs(root, 0, limit);
        if (n < limit) { return null; }
        return root;
    }

    static int Dfs(TreeNode node, int curr, int limit)
    {
        if (node is null) { return int.MinValue; }
        var sum = node.val + curr;
        if (node.left is null && node.right is null) { return sum; }
        var left = Dfs(node.left, sum, limit);
        var right = Dfs(node.right, sum, limit);
        node.left = left < limit ? null : node.left;
        node.right = right < limit ? null : node.right;
        return Math.Max(left, right);
    }
}

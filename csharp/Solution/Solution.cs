// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int FindSecondMinimumValue(TreeNode root)
    {
        return Dfs(root, root.val);
    }

    static int Dfs(TreeNode node, int val)
    {
        if (node is null) { return -1; }
        if (node.val > val) { return node.val; }
        var left = Dfs(node.left, val);
        var right = Dfs(node.right, val);
        return (left == -1, right == -1) switch
        {
            (true, true) => -1,
            (true, _) => right,
            (_, true) => left,
            _ => Math.Min(left, right)
        };
    }
}

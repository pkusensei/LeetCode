using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int GoodNodes(TreeNode root)
    {
        if (root is null) { return 0; }
        return Dfs(root, root.val);
    }

    static int Dfs(TreeNode node, int max)
    {
        if (node is null) { return 0; }
        var res = node.val >= max ? 1 : 0;
        max = Math.Max(max, node.val);
        return res + Dfs(node.left, max) + Dfs(node.right, max);
    }
}

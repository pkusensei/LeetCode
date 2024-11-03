using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode IncreasingBST(TreeNode root)
    {
        return Dfs(root).head;
    }

    static (TreeNode head, TreeNode tail) Dfs(TreeNode node)
    {
        if (node is null) { return (null, null); }
        var (h1, t1) = Dfs(node.left);
        var (h2, t2) = Dfs(node.right);
        if (h1 is not null)
        {
            var head = h1;
            t1.right = node;
            node.left = null;
            node.right = h2;
            var tail = t2 ?? node;
            return (head, tail);
        }
        else
        {
            var head = node;
            node.right = h2;
            var tail = t2 ?? node;
            return (head, tail);
        }
    }
}

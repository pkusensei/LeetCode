using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Rob(TreeNode root)
    {
        (int inc, int exc) = Dfs(root);
        return int.Max(inc, exc);

        static (int inc, int exc) Dfs(TreeNode node)
        {
            if (node is null) { return (0, 0); }
            var left = Dfs(node.left);
            var right = Dfs(node.right);
            int inc = node.val + left.exc + right.exc;
            int exc = int.Max(left.inc, left.exc) + int.Max(right.inc, right.exc);
            return (inc, exc);
        }
    }
}

using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int AverageOfSubtree(TreeNode root)
    {
        return Dfs(root).res;

        static (int sum, int count, int res) Dfs(TreeNode node)
        {
            if (node is null) { return (0, 0, 0); }
            var left = Dfs(node.left);
            var right = Dfs(node.right);
            var sum = node.val + left.sum + right.sum;
            var count = 1 + left.count + right.count;
            var curr = sum / count == node.val ? 1 : 0;
            return (sum, count, curr + left.res + right.res);
        }
    }
}

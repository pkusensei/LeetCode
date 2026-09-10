using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

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
            int sum = node.val + left.sum + right.sum;
            int count = 1 + left.count + right.count;
            int res = left.res + right.res;
            res += sum / count == node.val ? 1 : 0;
            return (sum, count, res);
        }
    }
}

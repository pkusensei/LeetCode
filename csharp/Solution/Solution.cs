using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int DistributeCoins(TreeNode root)
    {
        int res = 0;
        Dfs(root);
        return res;

        (int sum, int count) Dfs(TreeNode node)
        {
            if (node is null) { return (0, 0); }
            var left = Dfs(node.left);
            var right = Dfs(node.right);
            int sum = left.sum + right.sum + node.val;
            int count = left.count + right.count + 1;
            res += int.Abs(sum - count);
            return (sum, count);
        }
    }
}

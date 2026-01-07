using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxProduct(TreeNode root)
    {
        long sum = Sum(root);
        long res = 0;
        Dfs(root);
        return (int)(res % 1_000_000_007);

        long Dfs(TreeNode node)
        {
            if (node is null) { return 0; }
            long left = Dfs(node.left);
            long right = Dfs(node.right);
            long curr = sum - node.val - left - right;
            res = long.Max(res, curr * (sum - curr));
            return node.val + left + right;
        }

        static long Sum(TreeNode node)
           => node is null ? 0 : node.val + Sum(node.left) + Sum(node.right);
    }
}

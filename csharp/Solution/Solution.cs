using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxProduct(TreeNode root)
    {
        const long MOD = 1_000_000_007;

        HashSet<long> vals = [];
        var sum = Dfs(root);
        return (int)(vals.Select(v => v * (sum - v)).Max() % MOD);

        long Dfs(TreeNode node)
        {
            if (node is null) { return 0; }
            long curr = node.val + Dfs(node.left) + Dfs(node.right);
            vals.Add(curr);
            return curr;
        }
    }
}
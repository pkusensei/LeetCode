using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;


public class Solution
{
    public int KthSmallest(TreeNode root, int k)
    {
        int? res = null;
        Dfs(root, k);
        return res.Value;

        int Dfs(TreeNode node, int k)
        {
            if (node is null || res.HasValue) { return 0; }
            int left = Dfs(node.left, k);
            k -= left;
            if (k == 1 && !res.HasValue) { res = node.val; }
            return left + 1 + Dfs(node.right, k - 1);
        }
    }
}
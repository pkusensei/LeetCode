using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxPathSum(TreeNode root)
    {
        int res = int.MinValue;
        Dfs(root);
        return res;

        int Dfs(TreeNode node)
        {
            if (node is null) { return 0; }
            int v1 = node.val;
            int left = Dfs(node.left);
            int right = Dfs(node.right);
            int v2 = node.val + left;
            int v3 = node.val + right;
            int v4 = node.val + left + right; // trimmed when recursing back up
            int val = int.Max(int.Max(v1, v2), int.Max(v3, v4));
            res = int.Max(res, val);
            return int.Max(v1, int.Max(v2, v3));
        }
    }
}
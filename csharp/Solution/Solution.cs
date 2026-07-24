using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool BtreeGameWinningMove(TreeNode root, int n, int x)
    {
        int leftv = 0;
        int rightv = 0;
        Dfs(root);
        int p = n - leftv - rightv - 1;
        return leftv > n / 2 || rightv > n / 2 || p > n / 2;

        int Dfs(TreeNode node)
        {
            if (node is null) { return 0; }
            int left = Dfs(node.left);
            int right = Dfs(node.right);
            if (x == node.val)
            {
                leftv = left;
                rightv = right;
            }
            return 1 + left + right;
        }
    }
}

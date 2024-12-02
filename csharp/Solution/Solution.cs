using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool BtreeGameWinningMove(TreeNode root, int n, int x)
    {
        int left_count = -1;
        int right_count = -1;
        Dfs(root);
        return left_count > n / 2 || right_count > n / 2 || n - left_count - right_count - 1 > n / 2;

        int Dfs(TreeNode node)
        {
            if (node is null) { return 0; }
            var left = Dfs(node.left);
            var right = Dfs(node.right);
            if (x == node.val)
            {
                left_count = left;
                right_count = right;
            }
            return left + right + 1;
        }
    }
}

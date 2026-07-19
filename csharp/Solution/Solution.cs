using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountDominantNodes(TreeNode root)
    {
        int res = 0;
        Dfs(root);
        return res;

        int Dfs(TreeNode node)
        {
            if (node is null) { return 0; }
            int left = Dfs(node.left);
            int right = Dfs(node.right);
            int curr = int.Max(node.val, int.Max(left, right));
            if (curr == node.val) { res += 1; }
            return curr;
        }
    }
}

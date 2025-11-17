using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinDiffInBST(TreeNode root)
    {
        int res = int.MaxValue;
        int? prev = null;
        Dfs(root);
        return res;

        void Dfs(TreeNode node)
        {
            if (node is null) { return; }
            Dfs(node.left);
            if (prev is not null)
            {
                res = int.Min(res, node.val - prev.Value);
            }
            prev = node.val;
            Dfs(node.right);
        }
    }
}


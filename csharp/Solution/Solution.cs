using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxAncestorDiff(TreeNode root)
    {
        int res = 0;
        Dfs(root.left, root.val, root.val);
        Dfs(root.right, root.val, root.val);
        return res;

        void Dfs(TreeNode node, int min, int max)
        {
            if (node is null) { return; }
            int a = int.Abs(min - node.val);
            int b = int.Abs(max - node.val);
            res = int.Max(res, int.Max(a, b));
            Dfs(node.left, int.Min(min, node.val), int.Max(max, node.val));
            Dfs(node.right, int.Min(min, node.val), int.Max(max, node.val));
        }
    }
}


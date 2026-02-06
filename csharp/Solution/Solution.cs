using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode SubtreeWithAllDeepest(TreeNode root)
    {
        return Dfs(root, 0).Item1;

        static (TreeNode, int) Dfs(TreeNode node, int depth)
        {
            if (node is null) { return (null, depth); }
            var (left, d1) = Dfs(node.left, 1 + depth);
            var (right, d2) = Dfs(node.right, 1 + depth);
            if (d1 < d2) { return (right, d2); }
            else if (d1 > d2) { return (left, d1); }
            else { return (node, d1); }
        }
    }
}
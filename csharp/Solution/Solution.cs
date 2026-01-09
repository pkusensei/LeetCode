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
        return Dfs(root).node;

        static (int depth, TreeNode node) Dfs(TreeNode node)
        {
            if (node is null) { return (0, null); }
            (int left_depth, TreeNode left) = Dfs(node.left);
            (int right_depth, TreeNode right) = Dfs(node.right);
            if (left_depth == right_depth) { return (1 + left_depth, node); }
            else if (left_depth < right_depth) { return (1 + right_depth, right); }
            else { return (1 + left_depth, left); }
        }
    }
}

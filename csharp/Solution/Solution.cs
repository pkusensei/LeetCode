using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode SufficientSubset(TreeNode root, int limit)
    {
        return Dfs(root, 0);

        TreeNode Dfs(TreeNode node, int val)
        {
            if (node is null) { return null; }
            val += node.val;
            if (node.left is null && node.right is null)
            {
                return val < limit ? null : node;
            }
            node.left = Dfs(node.left, val);
            node.right = Dfs(node.right, val);
            return node.left is null && node.right is null ? null : node;
        }
    }
}

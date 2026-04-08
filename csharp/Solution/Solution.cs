using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsUnivalTree(TreeNode root)
    {
        return Dfs(root, null);
        
        static bool Dfs(TreeNode node, int? val)
        {
            if (node is null) { return true; }
            val ??= node.val;
            return node.val == val.Value && Dfs(node.left, val) && Dfs(node.right, val);
        }
    }
}

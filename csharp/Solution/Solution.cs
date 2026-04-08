using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinCameraCover(TreeNode root)
    {
        HashSet<TreeNode> seen = [null];
        return Dfs(root, null);

        int Dfs(TreeNode node, TreeNode parent)
        {
            if (node is null) { return 0; }
            int res = Dfs(node.left, node) + Dfs(node.right, node);
            if (!seen.Contains(node.left) || !seen.Contains(node.right)
            || (!seen.Contains(node) && parent is null))
            {
                seen.UnionWith([node.left, node.right, node, parent]);
                res += 1;
            }
            return res;
        }
    }
}

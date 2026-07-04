using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<TreeNode> DelNodes(TreeNode root, int[] to_delete)
    {
        List<TreeNode> res = [];
        HashSet<int> set = [.. to_delete];
        var v = Dfs(root);
        if (v is not null) { res.Add(v); }
        return res;

        TreeNode Dfs(TreeNode node)
        {
            if (node is null) { return null; }
            node.left = Dfs(node.left);
            node.right = Dfs(node.right);
            if (set.Contains(node.val))
            {
                if (node.left is not null) { res.Add(node.left); }
                if (node.right is not null) { res.Add(node.right); }
                return null;
            }
            else
            {
                return node;
            }
        }
    }
}

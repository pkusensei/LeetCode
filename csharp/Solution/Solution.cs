using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<TreeNode> DelNodes(TreeNode root, int[] to_delete)
    {
        var res = Dfs(root, to_delete).tree;
        if (!to_delete.Contains(root.val)) { res.Add(root); }
        return res;
    }

    static (TreeNode node, List<TreeNode> tree) Dfs(TreeNode node, int[] dels)
    {
        if (node is null) { return (null, null); }
        List<TreeNode> res = [];
        var left = Dfs(node.left, dels);
        var right = Dfs(node.right, dels);
        if (left.tree is not null) { res.AddRange(left.tree); }
        if (right.tree is not null) { res.AddRange(right.tree); }
        if (dels.Contains(node.val))
        {
            if (left.node is not null) { res.Add(left.node); }
            if (right.node is not null) { res.Add(right.node); }
            return (null, res);
        }
        else
        {
            node.left = left.node;
            node.right = right.node;
            return (node, res);
        }
    }
}

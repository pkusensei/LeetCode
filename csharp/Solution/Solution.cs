using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode LcaDeepestLeaves(TreeNode root)
    {
        Dictionary<TreeNode, int> depths = [];
        var depth = Depth(root, 0);
        // return Find(root);

        return Dfs(root, 0).node;

        int Depth(TreeNode node, int depth)
        {
            if (node is null) { return depth - 1; }
            var left = Depth(node.left, 1 + depth);
            var right = Depth(node.right, 1 + depth);
            var d = Math.Max(left, right);
            depths.Add(node, d);
            return d;
        }

        TreeNode Find(TreeNode node)
        {
            if (node is null) { return null; }
            if (depths[node] == depth)
            {
                var left = node.left is not null && depths[node.left] == depth;
                var right = node.right is not null && depths[node.right] == depth;
                return (left, right) switch
                {
                    (true, false) => Find(node.left),
                    (false, true) => Find(node.right),
                    _ => node,
                };
            }
            return null;
        }
    }

    static (TreeNode node, int depth) Dfs(TreeNode node, int depth)
    {
        if (node is null) { return (node, depth); }
        var left = Dfs(node.left, 1 + depth);
        var right = Dfs(node.right, 1 + depth);
        if (left.depth == right.depth) { return (node, left.depth); }
        return left.depth > right.depth ? left : right;
    }
}

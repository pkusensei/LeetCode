using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode SubtreeWithAllDeepest(TreeNode root)
    {
        var dict = new Dictionary<TreeNode, int>();
        var target = FindDepth(root, 0, dict);
        return Dfs(root, target, dict);
    }

    static TreeNode Dfs(TreeNode node, int target, Dictionary<TreeNode, int> dict)
    {
        if (node is null) { return null; }
        if (dict[node] == target)
        {
            var left = node.left is not null && dict.TryGetValue(node.left, out var v1) && v1 == target;
            var right = node.right is not null && dict.TryGetValue(node.right, out var v2) && v2 == target;
            return (left, right) switch
            {
                (true, false) => Dfs(node.left, target, dict),
                (false, true) => Dfs(node.right, target, dict),
                _ => node,
            };
        }
        return null;
    }

    static int FindDepth(TreeNode node, int depth, Dictionary<TreeNode, int> dict)
    {
        if (node is null) { return depth - 1; }
        var a = FindDepth(node.left, 1 + depth, dict);
        var b = FindDepth(node.right, 1 + depth, dict);
        var v = Math.Max(a, b);
        dict.Add(node, v);
        return v;
    }
}

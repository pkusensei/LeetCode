using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<int> DistanceK(TreeNode root, TreeNode target, int k)
    {
        List<int> res = [];
        if (root is null) { return res; }

        Dictionary<TreeNode, TreeNode> parents = [];
        FindParent(root, null, parents);
        Dfs(target, k, res, parents, []);
        return res;
    }

    static void Dfs(TreeNode node, int k, List<int> nums, Dictionary<TreeNode, TreeNode> parents, HashSet<int> seen)
    {
        if (node is null || !seen.Add(node.val)) { return; }
        if (k == 0) { nums.Add(node.val); return; }
        Dfs(node.left, k - 1, nums, parents, seen);
        Dfs(node.right, k - 1, nums, parents, seen);
        if (parents.TryGetValue(node, out var p))
        {
            Dfs(p, k - 1, nums, parents, seen);
        }
    }

    static void FindParent(TreeNode node, TreeNode parent, Dictionary<TreeNode, TreeNode> parents)
    {
        if (node is null) { return; }
        parents.Add(node, parent);
        FindParent(node.left, node, parents);
        FindParent(node.right, node, parents);
    }
}

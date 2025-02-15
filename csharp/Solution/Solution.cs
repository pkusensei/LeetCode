using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode CanMerge(IList<TreeNode> trees)
    {
        Dictionary<int, TreeNode> val_roots = [];
        HashSet<int> non_roots = [];
        // Find all root_val-root and non root vals
        foreach (var (i, tree) in trees.Select((t, i) => (i, t)))
        {
            val_roots.Add(tree.val, tree);
            var leaves = CollectVals(tree);
            leaves.RemoveAt(leaves.Count - 1);
            non_roots.UnionWith(leaves);
        }
        var roots = val_roots.Keys.Where(k => !non_roots.Contains(k))
            .Select(k => val_roots[k]).Take(2).ToList();
        if (roots.Count != 1) { return null; }
        var root = roots.Single();
        TreeNode dummy = new(0, root);
        HashSet<int> used_roots = [root.val];
        root = Merge(root, dummy, false);
        var all_vals = CollectVals(root); // All vals must be merged
        if (Check(root, 0, 50_001) && all_vals.Count == non_roots.Count + 1) { return root; }
        return null;

        static bool Check(TreeNode node, int min, int max)
        {
            if (node is null) { return true; }
            if (node.val <= min || node.val >= max) { return false; }
            return Check(node.left, min, node.val) && Check(node.right, node.val, max);
        }

        TreeNode Merge(TreeNode node, TreeNode prev, bool is_left)
        {
            if (node is null) { return null; }
            if (node.left is null && node.right is null)
            {
                if (val_roots.TryGetValue(node.val, out var new_root) && used_roots.Add(node.val))
                {
                    node = new_root;
                    if (is_left) { prev.left = new_root; }
                    else { prev.right = new_root; }
                }
            }
            var left = Merge(node.left, node, true);
            var right = Merge(node.right, node, false);
            node.left = left;
            node.right = right;
            return node;
        }

        static List<int> CollectVals(TreeNode node)
        {
            if (node is null) { return []; }
            var res = CollectVals(node.left);
            res.AddRange(CollectVals(node.right));
            res.Add(node.val); // Post-order
            return res;
        }
    }
}

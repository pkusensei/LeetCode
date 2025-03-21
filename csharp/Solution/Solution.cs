using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] TreeQueries(TreeNode root, int[] queries)
    {
        Dictionary<int, int> cache = [];
        Dictionary<int, int> heights = [];
        Dfs(root, 0, 0);
        return [.. queries.Select(n => cache[n])];

        void Dfs(TreeNode node, int depth, int max_val)
        {
            if (node is null) { return; }
            cache.Add(node.val, max_val);
            Dfs(node.left, 1 + depth, Math.Max(max_val, 1 + depth + Height(node.right)));
            Dfs(node.right, 1 + depth, Math.Max(max_val, 1 + depth + Height(node.left)));
        }

        int Height(TreeNode node)
        {
            if (node is null) { return -1; }
            if (heights.TryGetValue(node.val, out var v)) { return v; }
            int res = 1 + Math.Max(Height(node.left), Height(node.right));
            heights.Add(node.val, res);
            return res;
        }
    }
}

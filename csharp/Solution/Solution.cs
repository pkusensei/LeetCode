using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode CreateBinaryTree(int[][] descriptions)
    {
        Dictionary<int, List<(int val, bool is_left)>> adj = [];
        HashSet<int> nodes = []; // nodes with parent
        foreach (var desc in descriptions)
        {
            var parent = desc[0];
            var child = desc[1];
            bool is_left = desc[2] == 1;
            if (!adj.TryAdd(parent, [(child, is_left)])) { adj[parent].Add((child, is_left)); }
            nodes.Add(child);
        }
        int root = adj.Keys.Where(k => !nodes.Contains(k)).Single();
        return Dfs(root);

        TreeNode Dfs(int val)
        {
            TreeNode node = new(val);
            if (adj.TryGetValue(val, out var nodes))
            {
                foreach (var item in nodes)
                {
                    if (item.is_left) { node.left = Dfs(item.val); }
                    else { node.right = Dfs(item.val); }
                }
            }
            return node;
        }
    }
}

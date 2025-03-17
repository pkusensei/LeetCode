using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int AmountOfTime(TreeNode root, int start)
    {
        Dictionary<int, List<int>> adj = [];
        Dfs(root, null);
        Queue<(int node, int dist)> queue = [];
        queue.Enqueue((start, 0));
        int res = 0;
        HashSet<int> seen = [start];
        while (queue.TryDequeue(out var item))
        {
            res = Math.Max(res, item.dist);
            foreach (var next in adj[item.node])
            {
                if (seen.Add(next)) { queue.Enqueue((next, 1 + item.dist)); }
            }
        }
        return res;

        void Dfs(TreeNode node, TreeNode parent)
        {
            if (node is null) { return; }
            adj.TryAdd(node.val, []);
            if (parent is not null) { adj[node.val].Add(parent.val); }
            if (node.left is not null)
            {
                adj[node.val].Add(node.left.val);
                Dfs(node.left, node);
            }
            if (node.right is not null)
            {
                adj[node.val].Add(node.right.val);
                Dfs(node.right, node);
            }
        }
    }
}

// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<IList<int>> LevelOrder(Node root)
    {
        List<IList<int>> res = [];
        if (root is null) { return res; }
        var queue = new Queue<(Node node, int level)>();
        queue.Enqueue((root, 0));
        var curr = new List<int>();
        var currLevel = 0;
        while (queue.TryDequeue(out var item))
        {
            var (node, level) = item;
            if (level == currLevel) { curr.Add(node.val); }
            else
            {
                res.Add(curr[..]);
                curr.Clear();
                currLevel = level;
                curr.Add(node.val);
            }
            foreach (var c in node.children)
            {
                queue.Enqueue((c, level + 1));
            }
        }
        if (curr.Count > 0) { res.Add(curr); }
        return res;
    }
}

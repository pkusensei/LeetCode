// using Solution.LList;
using System.Text;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<double> AverageOfLevels(TreeNode root)
    {
        var queue = new Queue<TreeNode>();
        queue.Enqueue(root);
        List<double> res = [];
        while (queue.Count > 0)
        {
            var nodes = queue.ToList();
            res.Add(nodes.Select(n => (long)n.val).Sum() / (double)nodes.Count);
            queue.Clear();
            foreach (var n in nodes)
            {
                if (n.left is not null) { queue.Enqueue(n.left); }
                if (n.right is not null) { queue.Enqueue(n.right); }
            }
        }
        return res;
    }
}

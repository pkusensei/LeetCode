using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<int> LargestValues(TreeNode root)
    {
        if (root is null) { return null; }
        var queue = new Queue<(TreeNode, int)>();
        queue.Enqueue((root, 0));
        var (level, val) = (0, root.val);
        var res = new List<int>();
        while (queue.TryDequeue(out var item))
        {
            var (node, curr_level) = item;
            if (node is not null)
            {
                if (curr_level > level)
                {
                    level = curr_level;
                    res.Add(val);
                    val = node.val;
                }
                else
                {
                    val = Math.Max(val, node.val);
                }
                queue.Enqueue((node.left, curr_level + 1));
                queue.Enqueue((node.right, curr_level + 1));
            }
        }
        res.Add(val);
        return res;
    }
}
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int FindBottomLeftValue(TreeNode root)
    {
        var queue = new Queue<(TreeNode, int)>();
        queue.Enqueue((root, 0));
        var (level, res) = (0, root.val);
        while (queue.TryDequeue(out var item))
        {
            var (node, curr_level) = item;
            if (node is not null)
            {
                if (curr_level > level)
                {
                    level = curr_level;
                    res = node.val;
                }
                queue.Enqueue((node.left, curr_level + 1));
                queue.Enqueue((node.right, curr_level + 1));
            }
        }
        return res;
    }
}
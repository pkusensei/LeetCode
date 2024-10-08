// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int WidthOfBinaryTree(TreeNode root)
    {
        var queue = new Queue<(TreeNode node, long idx)>();
        queue.Enqueue((root, 0));
        long res = 0;
        while (queue.TryPeek(out var item))
        {
            var left_most = item.idx;
            var right_most = long.MinValue;
            var count = queue.Count;
            for (int i = 0; i < count; i++)
            {
                var (node, idx) = queue.Dequeue();
                right_most = Math.Max(right_most, idx);
                if (node.left is not null)
                {
                    queue.Enqueue((node.left, 2 * idx + 1));
                }
                if (node.right is not null)
                {
                    queue.Enqueue((node.right, 2 * idx + 2));
                }
            }
            res = Math.Max(res, right_most - left_most + 1);
        }
        return (int)res;
    }
}

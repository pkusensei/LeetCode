using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxLevelSum(TreeNode root)
    {
        int res = 1;
        Queue<TreeNode> queue = new();
        queue.Enqueue(root);
        int level = 0;
        int max = int.MinValue;
        while (queue.Count > 0)
        {
            level += 1;
            int n = queue.Count;
            int sum = 0;
            for (int _ = 0; _ < n; _++)
            {
                var node = queue.Dequeue();
                sum += node.val;
                if (node.left is not null) { queue.Enqueue(node.left); }
                if (node.right is not null) { queue.Enqueue(node.right); }
            }
            if (sum > max)
            {
                max = sum;
                res = level;
            }
        }
        return res;
    }
}

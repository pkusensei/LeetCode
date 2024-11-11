using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool IsCompleteTree(TreeNode root)
    {
        if (root is null) { return true; }
        var queue = new Queue<TreeNode>();
        queue.Enqueue(root);
        while (queue.TryDequeue(out var node))
        {
            if (node is null) { return queue.All(n => n is null); }
            else
            {
                queue.Enqueue(node.left);
                queue.Enqueue(node.right);
            }
        }
        return true;
    }
}

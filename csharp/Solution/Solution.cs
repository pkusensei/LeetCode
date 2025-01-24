using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool IsEvenOddTree(TreeNode root)
    {
        if (root is null) { return true; }
        Queue<TreeNode> queue = [];
        queue.Enqueue(root);
        var is_even = true;
        while (queue.Count > 0)
        {
            var n = queue.Count;
            TreeNode prev = null;
            for (int _ = 0; _ < n; _++)
            {
                var node = queue.Dequeue();
                if (is_even != ((node.val & 1) == 1)) { return false; }
                if (prev is null || (is_even && prev.val < node.val) || (!is_even && prev.val > node.val))
                {
                    prev = node;
                }
                else { return false; }
                if (node.left is not null) { queue.Enqueue(node.left); }
                if (node.right is not null) { queue.Enqueue(node.right); }
            }
            is_even = !is_even;
        }
        return true;
    }
}

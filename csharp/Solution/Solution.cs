// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode ConstructMaximumBinaryTree(int[] nums)
    {
        if (nums.Length == 0) { return null; }
        var curr = nums.Max();
        var idx = nums.ToList().IndexOf(curr);
        var left = ConstructMaximumBinaryTree(nums[..idx]);
        var right = ConstructMaximumBinaryTree(nums[(idx + 1)..]);
        return new(curr, left, right);
    }

    public TreeNode WithStack(int[] nums)
    {
        var stack = new Stack<TreeNode>();
        foreach (var num in nums)
        {
            TreeNode node = new(num);
            while (stack.TryPeek(out var n1) && n1.val < num)
            {
                node.left = stack.Pop();
            }
            if (stack.TryPeek(out var n))
            {
                n.right = node;
            }
            stack.Push(node);
        }
        return stack.Last();
    }
}

using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int GetMinimumDifference(TreeNode root)
    {
        var nums = new List<int>();
        Inorder(root, nums);
        return nums.Skip(1).Zip(nums).Select(p => p.First - p.Second).Min();
    }

    static void Inorder(TreeNode node, List<int> nums)
    {
        if (node is null) { return; }
        Inorder(node.left, nums);
        nums.Add(node.val);
        Inorder(node.right, nums);
    }
}
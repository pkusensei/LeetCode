using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Codec
{
    // Encodes a tree to a single string.
    public string serialize(TreeNode root)
    {
        StringBuilder sb = new();
        Preorder(root);
        if (sb.Length > 0) { sb.Remove(sb.Length - 1, 1); }
        return sb.ToString();

        void Preorder(TreeNode node)
        {
            if (node is null) { return; }
            sb.Append(node.val);
            sb.Append(',');
            Preorder(node.left);
            Preorder(node.right);
        }
    }

    // Decodes your encoded data to tree.
    public TreeNode deserialize(string data)
    {
        if (string.IsNullOrEmpty(data)) { return null; }
        int[] nums = [.. data.Split(',').Select(int.Parse)];
        return Dfs(nums);

        static TreeNode Dfs(ReadOnlySpan<int> nums)
        {
            if (nums.IsEmpty) { return null; }
            int val = nums[0];
            int i = 1;
            for (; i < nums.Length && nums[i] < val; i++) { }
            var left = Dfs(nums[1..i]);
            var right = Dfs(nums[i..]);
            return new(val, left, right);
        }
    }
}
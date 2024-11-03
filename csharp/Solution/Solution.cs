using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode ConstructFromPrePost(int[] preorder, int[] postorder)
    {
        if (preorder.Length * postorder.Length == 0) { return null; }
        TreeNode root = new(postorder.Last());
        if (postorder.Length == 1) { return root; }

        var right_root = postorder.Reverse().Skip(1).First();
        var right_idx_pre = Array.FindIndex(preorder, num => num == right_root);
        var right_pre = preorder[right_idx_pre..];
        var right_post = postorder.Where(num => right_pre.Contains(num)).ToArray();

        var left_root = preorder.Skip(1).First();
        var left_pre = preorder[1..right_idx_pre];
        var left_post = postorder.Where(num => left_pre.Contains(num)).ToArray();

        var left = ConstructFromPrePost(left_pre, left_post);
        var right = ConstructFromPrePost(right_pre, right_post);
        root.left = left; root.right = right;
        return root;
    }
}

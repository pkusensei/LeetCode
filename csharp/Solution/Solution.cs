using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode BuildTree(int[] preorder, int[] inorder)
    {
        return Dfs(preorder, inorder);

        static TreeNode Dfs(ReadOnlySpan<int> pre, ReadOnlySpan<int> inorder)
        {
            if (pre.IsEmpty) { return null; }
            TreeNode root = new(pre[0]);
            if (pre.Length == 1) { return root; }
            int root_idx = 0;
            for (root_idx = 0; root_idx < inorder.Length; root_idx += 1)
            {
                if (inorder[root_idx] == pre[0]) { break; }
            }
            var left_in = inorder[..root_idx];
            var right_in = inorder[(1 + root_idx)..];
            var left_pre = pre[1..(1 + root_idx)];
            var right_pre = pre[(1 + root_idx)..];
            root.left = Dfs(left_pre, left_in);
            root.right = Dfs(right_pre, right_in);
            return root;
        }
    }
}
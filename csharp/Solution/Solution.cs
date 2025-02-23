using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode ConstructFromPrePost(int[] preorder, int[] postorder)
    {
        return Dfs(preorder, postorder);

        TreeNode Dfs(int[] pre, int[] post)
        {
            if (pre.Length == 0) { return null; }
            TreeNode root = new(pre[0]);
            if (pre.Length == 1) { return root; }
            var right_val = post[^2];
            var right_idx_in_pre = Array.IndexOf(pre, right_val);
            var right_pre = pre[right_idx_in_pre..];
            var right_post = post.Where(n => right_pre.Contains(n));
            root.right = Dfs(right_pre, [.. right_post]);
            var left_pre = pre[1..right_idx_in_pre];
            var left_post = post.Where(n => left_pre.Contains(n));
            root.left = Dfs(left_pre, [.. left_post]);
            return root;
        }
    }

    public TreeNode TwoPtrs(int[] pre, int[] post)
    {
        int pre_idx = 0;
        int post_idx = 0;
        return Dfs();

        TreeNode Dfs()
        {
            TreeNode root = new(pre[pre_idx]);
            pre_idx += 1;
            if (root.val != post[post_idx]) { root.left = Dfs(); }
            if (root.val != post[post_idx]) { root.right = Dfs(); }
            post_idx += 1;
            return root;
        }
    }
}


using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode LcaDeepestLeaves(TreeNode root)
    {
        int max_depth = 0;
        var res = root;
        PostOrder(root, 0);
        return res;

        int PostOrder(TreeNode node, int depth)
        {
            if (node is null) { return depth; }
            int left = PostOrder(node.left, 1 + depth);
            int right = PostOrder(node.right, 1 + depth);
            int curr = Math.Max(left, right);
            if (curr > max_depth) { max_depth = curr; }
            if (left == max_depth && right == max_depth) { res = node; }
            return curr;
        }
    }
}

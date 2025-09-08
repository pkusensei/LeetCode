using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindTilt(TreeNode root)
    {
        int res = 0;
        PostOrder(root);
        return res;

        int PostOrder(TreeNode node)
        {
            if (node is null) { return 0; }
            int left = PostOrder(node.left);
            int right = PostOrder(node.right);
            res += int.Abs(left - right);
            return node.val + left + right;
        }
    }
}
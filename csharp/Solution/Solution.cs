using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int SumEvenGrandparent(TreeNode root)
    {
        if (root is null) { return 0; }
        var res = SumSubtree(root);
        res += SumEvenGrandparent(root.left) + SumEvenGrandparent(root.right);
        return res;

        static int SumSubtree(TreeNode node)
        {
            if (node is null || (node.val & 1) == 1) { return 0; }
            var res = 0;
            if (node.left is not null)
            {
                res += node.left.left is null ? 0 : node.left.left.val;
                res += node.left.right is null ? 0 : node.left.right.val;
            }
            if (node.right is not null)
            {
                res += node.right.left is null ? 0 : node.right.left.val;
                res += node.right.right is null ? 0 : node.right.right.val;
            }
            return res;
        }
    }
}
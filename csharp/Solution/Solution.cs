// using Solution.LList;
using System.Text;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode MergeTrees(TreeNode root1, TreeNode root2)
    {
        switch (root1 is null, root2 is null)
        {
            case (true, true): return null;
            case (true, false): return root2;
            case (false, true): return root1;
            default:
                var left = MergeTrees(root1.left, root2.left);
                var right = MergeTrees(root1.right, root2.right);
                return new(root1.val + root2.val, left, right);
        }
    }
}

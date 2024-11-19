using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode BstFromPreorder(int[] preorder)
    {
        if (preorder.Length == 0) { return null; }
        var val = preorder[0];
        var left = preorder.Where(v => v < val).ToArray();
        var right = preorder.Where(v => v > val).ToArray();
        TreeNode root = new(val, BstFromPreorder(left), BstFromPreorder(right));
        return root;
    }
}

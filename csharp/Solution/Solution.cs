// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int FindTilt(TreeNode root)
    {
        Tilt(root);
        return Sum(root);
    }

    static int Tilt(TreeNode node)
    {
        if (node is null) { return 0; }
        var left = Tilt(node.left);
        var right = Tilt(node.right);
        var temp = node.val + left + right;
        node.val = Math.Abs(left - right);
        return temp;
    }

    static int Sum(TreeNode node)
    {
        if (node is null) { return 0; }
        return node.val + Sum(node.left) + Sum(node.right);
    }
}

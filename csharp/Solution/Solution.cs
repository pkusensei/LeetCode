using Solution.Tree;

namespace Solution;

public class Solution
{
    public int Rob(TreeNode root)
    {
        var res = Solve(root);
        return Math.Max(res.inc, res.exc);
    }

    (int inc, int exc) Solve(TreeNode root)
    {
        if (root is null) { return (0, 0); }
        var left = Solve(root.left);
        var right = Solve(root.right);
        var inc = left.exc + right.exc + root.val;
        var exc = Math.Max(left.inc, left.exc) + Math.Max(right.inc, right.exc);
        return (inc, exc);
    }
}

using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int SumOfLeftLeaves(TreeNode root)
    {
        return Solve(root, false);
    }

    static int Solve(TreeNode node, bool isLeft)
    {
        if (node is null) { return 0; }
        if (node.left is null && node.right is null && isLeft) { return node.val; }
        return Solve(node.left, true) + Solve(node.right, false);
    }
}

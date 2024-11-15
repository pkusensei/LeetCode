using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int DistributeCoins(TreeNode root)
    {
        return Dfs(root).moves;
    }

    (int flow, int moves) Dfs(TreeNode node)
    {
        if (node is null) { return (0, 0); }
        var left = Dfs(node.left);
        var right = Dfs(node.right);
        // net flow of current subtree
        // negative => flow in
        // positive => flow out
        // This flow needs to be addressed by node's parent
        var curr = left.flow + right.flow + node.val - 1;
        // Thus one recursion stack up,
        // the parent node accumulates all of its children's flows
        var moves = left.moves + right.moves + Math.Abs(left.flow) + Math.Abs(right.flow);
        return (curr, moves);
    }
}

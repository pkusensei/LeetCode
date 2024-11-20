using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int SumRootToLeaf(TreeNode root)
    {
        if (root is null) { return 0; }
        return Dfs(root, 0);
    }

    static int Dfs(TreeNode node, int curr)
    {
        if (node is null) { return 0; }
        curr = (curr << 1) | node.val;
        if (node.left is null && node.right is null) { return curr; }
        return Dfs(node.left, curr) + Dfs(node.right, curr);
    }
}

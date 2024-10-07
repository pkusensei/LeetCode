// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool FindTarget(TreeNode root, int k)
    {
        return Dfs(root, root, k);
    }

    static bool Dfs(TreeNode root, TreeNode taken, int k)
    {
        if (taken is null) { return false; }
        return Find(root, taken, k - taken.val) || Dfs(root, taken.left, k) || Dfs(root, taken.right, k);
    }

    static bool Find(TreeNode node, TreeNode taken, int k)
    {
        if (node is null) { return false; }
        if (k == node.val) { return node != taken; }
        else if (k < node.val) { return Find(node.left, taken, k); }
        else { return Find(node.right, taken, k); }
    }
}

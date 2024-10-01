// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool IsSubtree(TreeNode root, TreeNode subRoot)
    {
        return (root is null, subRoot is null)
        switch
        {
            (true, true) => true,
            (false, true) or (true, false) => false,
            _ => Dfs(root, subRoot) || IsSubtree(root.left, subRoot) || IsSubtree(root.right, subRoot),
        };

    }

    static bool Dfs(TreeNode a, TreeNode b)
    {
        return (a is null, b is null)
        switch
        {
            (true, true) => true,
            (false, true) or (true, false) => false,
            _ => a.val == b.val && Dfs(a.left, b.left) && Dfs(a.right, b.right),
        };
    }
}

using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsSubtree(TreeNode root, TreeNode subRoot)
    {
        if (root is null) { return subRoot is null; }
        return Dfs(root, subRoot)
            || IsSubtree(root.left, subRoot) || IsSubtree(root.right, subRoot);

        static bool Dfs(TreeNode node1, TreeNode node2)
        {
            if (node2 is null) { return node1 is null; }
            if (node1 is null) { return false; }
            return node1.val == node2.val
                && Dfs(node1.left, node2.left) && Dfs(node1.right, node2.right);
        }
    }
}

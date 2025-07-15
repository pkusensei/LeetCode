using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    TreeNode prev = new(int.MinValue);
    TreeNode n1 = null;
    TreeNode n2 = null;

    public void RecoverTree(TreeNode root)
    {
        Dfs(root);
        (n1.val, n2.val) = (n2.val, n1.val);
    }

    void Dfs(TreeNode node)
    {
        if (node is null) { return; }
        Dfs(node.left);
        if (n1 is null && prev.val > node.val) { n1 = prev; }
        if (n1 is not null && prev.val > node.val) { n2 = node; }
        prev = node;
        Dfs(node.right);
    }
}
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode AddOneRow(TreeNode root, int val, int depth)
    {
        TreeNode dummy = new(val, root);
        Dfs(dummy, val, depth);
        return dummy.left;

        static void Dfs(TreeNode node, int val, int depth)
        {
            if (node is null) { return; }
            if (depth == 1)
            {
                node.left = new(val, node.left);
                node.right = new(val, null, node.right);
            }
            else
            {
                Dfs(node.left, val, depth - 1);
                Dfs(node.right, val, depth - 1);
            }
        }
    }
}
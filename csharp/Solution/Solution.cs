using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode MergeTrees(TreeNode root1, TreeNode root2)
    {
        return Dfs(root1, root2);

        static TreeNode Dfs(TreeNode node1, TreeNode node2)
        {
            switch (node1 is null, node2 is null)
            {
                case (true, true): return null;
                case (true, false): return node2;
                case (false, true): return node1;
                default:
                    int val = node1.val + node2.val;
                    return new(val, Dfs(node1.left, node2.left), Dfs(node1.right, node2.right));
            }
        }
    }
}
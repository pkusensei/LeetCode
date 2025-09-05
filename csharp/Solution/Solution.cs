using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode ConvertBST(TreeNode root)
    {
        Dfs(root, 0);
        return root;

        static int Dfs(TreeNode node, int suffix)
        {
            if (node is null) { return suffix; }
            suffix = Dfs(node.right, suffix);
            node.val += suffix;
            return Dfs(node.left, node.val);
        }
    }
}

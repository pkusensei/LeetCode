using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode BstToGst(TreeNode root)
    {
        Dfs(root, 0);
        return root;
    }

    static int Dfs(TreeNode node, int num)
    {
        if (node is null) { return num; }
        var v = Dfs(node.right, num);
        node.val += v;
        return Dfs(node.left, node.val);
    }
}

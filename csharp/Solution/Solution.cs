using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode ConvertBST(TreeNode root)
    {
        Dfs(root, 0);
        return root;
    }

    static int Dfs(TreeNode node, int num)
    {
        if (node is null) { return num; }
        num = Dfs(node.right, num);
        node.val += num;
        num = Dfs(node.left, node.val);
        return num;
    }
}
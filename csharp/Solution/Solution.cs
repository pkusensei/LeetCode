// using Solution.LList;
using System.Text;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode AddOneRow(TreeNode root, int val, int depth)
    {
        return Dfs(root, val, depth, true);
    }

    static TreeNode Dfs(TreeNode root, int val, int depth, bool is_left)
    {
        if (root is null && depth > 1) { return null; }
        if (depth < 2)
        {
            var node = new TreeNode(val);
            if (is_left) { node.left = root; }
            else { node.right = root; }
            return node;
        }
        root.left = Dfs(root.left, val, depth - 1, true);
        root.right = Dfs(root.right, val, depth - 1, false);
        return root;
    }
}

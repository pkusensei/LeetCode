using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode RemoveLeafNodes(TreeNode root, int target)
    {
        if (root is null) { return null; }
        root.left = RemoveLeafNodes(root.left, target);
        root.right = RemoveLeafNodes(root.right, target);
        if (root.left is null && root.right is null && root.val == target) { return null; }
        return root;
    }
}
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode InsertIntoMaxTree(TreeNode root, int val)
    {
        if (root is null) { return new(val); }
        if (root.val < val) { return new(val, root, null); }
        root.right = InsertIntoMaxTree(root.right, val);
        return root;
    }
}

using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode InsertIntoBST(TreeNode root, int val)
    {
        if (root is null) { return new(val); }
        if (root.val < val) { root.right = InsertIntoBST(root.right, val); }
        else { root.left = InsertIntoBST(root.left, val); }
        return root;
    }
}

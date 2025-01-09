using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode GetTargetCopy(TreeNode original, TreeNode cloned, TreeNode target)
    {
        if (original is null) { return null; }
        if (object.ReferenceEquals(original, target)) { return cloned; }
        return GetTargetCopy(original.left, cloned.left, target) ?? GetTargetCopy(original.right, cloned.right, target);
    }
}

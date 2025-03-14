using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool EvaluateTree(TreeNode root) => root.val switch
    {
        0 => false,
        1 => true,
        2 => EvaluateTree(root.left) || EvaluateTree(root.right),
        _ => EvaluateTree(root.left) && EvaluateTree(root.right),
    };
}

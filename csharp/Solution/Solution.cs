using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode LowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q)
    {
        int min = int.Min(p.val, q.val);
        int max = int.Max(p.val, q.val);
        return Dfs(root, min, max);

        static TreeNode Dfs(TreeNode node, int min, int max)
        {
            int val = node.val;
            if (min <= val && val <= max) { return node; }
            if (val < min) { return Dfs(node.right, min, max); }
            return Dfs(node.left, min, max);
        }
    }
}
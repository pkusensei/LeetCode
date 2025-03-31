using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode ReplaceValueInTree(TreeNode root)
    {
        int[] level_sums = new int[100_000];
        SumUp(root, 0);
        Replace(root, 0, 0);
        return root;

        void SumUp(TreeNode node, int depth)
        {
            if (node is null) { return; }
            level_sums[depth] += node.val;
            SumUp(node.left, 1 + depth);
            SumUp(node.right, 1 + depth);
        }

        void Replace(TreeNode node, int depth, int sib_sum)
        {
            if (node is null) { return; }
            if (depth < 2) { node.val = 0; }
            else { node.val = level_sums[depth] - node.val - sib_sum; }
            int left_val = node.left is null ? 0 : node.left.val;
            int right_val = node.right is null ? 0 : node.right.val;
            Replace(node.left, 1 + depth, right_val);
            Replace(node.right, 1 + depth, left_val);
        }
    }
}

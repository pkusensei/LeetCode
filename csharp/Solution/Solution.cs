using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxSumBST(TreeNode root)
    {
        var min_sum = int.MinValue;
        Dfs(root);
        return min_sum < 0 ? 0 : min_sum;

        State? Dfs(TreeNode node)
        {
            if (node is null) { return null; }
            (var left, var right) = (Dfs(node.left), Dfs(node.right));
            State? res = null;
            if (!left.HasValue && !right.HasValue)
            {
                res = new State(true, node.val, node.val, node.val);
            }
            else if (left.HasValue && !right.HasValue)
            {
                bool is_bst = left.Value.IsBst && left.Value.MaxLeft < node.val;
                res = new State(is_bst, left.Value.Sum + node.val, node.val, left.Value.MinRight);
            }
            else if (!left.HasValue && right.HasValue)
            {
                bool is_bst = right.Value.IsBst && node.val < right.Value.MinRight;
                res = new State(is_bst, right.Value.Sum + node.val, right.Value.MaxLeft, node.val);
            }
            else
            {
                bool is_bst = left.Value.IsBst && left.Value.MaxLeft < node.val
                    && right.Value.IsBst && node.val < right.Value.MinRight;
                res = new State(is_bst, left.Value.Sum + right.Value.Sum + node.val,
                                right.Value.MaxLeft, left.Value.MinRight);
            }
            if (res.Value.IsBst) { min_sum = Math.Max(min_sum, res.Value.Sum); }
            return res;
        }
    }

}

public readonly record struct State(bool IsBst, int Sum, int MaxLeft, int MinRight);
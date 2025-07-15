using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> PathSum(TreeNode root, int targetSum)
    {
        List<IList<int>> res = [];
        Backtrack(root, targetSum, []);
        return res;

        void Backtrack(TreeNode node, int target, List<int> curr)
        {
            if (node is null) { return; }
            curr.Add(node.val);
            target -= node.val;
            if (node.left is null && node.right is null && 0 == target)
            {
                res.Add([.. curr]);
            }
            Backtrack(node.left, target, curr);
            Backtrack(node.right, target, curr);
            curr.RemoveAt(curr.Count - 1);
        }
    }
}
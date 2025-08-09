using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PathSum(TreeNode root, int targetSum)
    {
        if (root is null) { return 0; }
        return Dfs(root, targetSum) + PathSum(root.left, targetSum) + PathSum(root.right, targetSum);

        static int Dfs(TreeNode node, long target)
        {
            if (node is null) { return 0; }
            target -= node.val;
            int res = target == 0 ? 1 : 0;
            return res + Dfs(node.left, target) + Dfs(node.right, target);
        }
    }

    public int WithPrefixSum(TreeNode root, int targetSum)
    {
        Dictionary<int, int> prefix = new() { [0] = 1 };
        return Backtrack(root, 0, targetSum);

        int Backtrack(TreeNode node, int lastSum, int targetSum)
        {
            if (node is null) { return 0; }
            int curr = lastSum + node.val;
            if ((lastSum ^ curr) < 0 && (node.val ^ curr) < 0)
            {
                return 0; // checks overflow
            }
            int pathCount = prefix.GetValueOrDefault(curr - targetSum, 0);
            prefix[curr] = prefix.GetValueOrDefault(curr, 0) + 1;

            int leftPaths = Backtrack(node.left, curr, targetSum);
            int rightPaths = Backtrack(node.right, curr, targetSum);

            prefix[curr] -= 1;
            return pathCount + leftPaths + rightPaths;
        }
    }
}
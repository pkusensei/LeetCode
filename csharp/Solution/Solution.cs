// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int PathSum(TreeNode root, int targetSum)
    {
        // return WithPrefixSum(root, targetSum, 0, new Dictionary<long, int> { { 0, 1 } });
        if (root is null) { return 0; }
        return Dfs(root, targetSum) +
            PathSum(root.left, targetSum) + PathSum(root.right, targetSum);
    }

    static int Dfs(TreeNode node, long curr)
    {
        if (node is null) { return 0; }
        var left = Dfs(node.left, curr - node.val);
        var right = Dfs(node.right, curr - node.val);
        if (node.val == curr) { return 1 + left + right; }
        else { return left + right; }
    }

    static int WithPrefixSum(TreeNode node, int target, long curr, IDictionary<long, int> prefix)
    {
        if (node is null) { return 0; }
        curr += node.val;
        int count = prefix.TryGetValue(curr - target, out int value) ? value : 0;

        if (prefix.ContainsKey(curr)) { prefix[curr] += 1; }
        else { prefix[curr] = 1; }

        count += WithPrefixSum(node.right, target, curr, prefix);
        count += WithPrefixSum(node.left, target, curr, prefix);

        prefix[curr] -= 1;
        return count;
    }
}

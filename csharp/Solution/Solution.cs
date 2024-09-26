using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] FindFrequentTreeSum(TreeNode root)
    {
        var count = new Dictionary<int, int>();
        Dfs(root, count);
        var maxCount = count.Values.Max();
        List<int> res = [];
        foreach (var (k, v) in count)
        {
            if (v == maxCount) { res.Add(k); }
        }
        return [.. res];
    }

    static int Dfs(TreeNode node, IDictionary<int, int> count)
    {
        var sum = node.val;
        if (node.left is not null) { sum += Dfs(node.left, count); }
        if (node.right is not null) { sum += Dfs(node.right, count); }
        if (count.TryGetValue(sum, out var v)) { count[sum] = v + 1; }
        else { count.Add(sum, 1); }
        return sum;
    }
}
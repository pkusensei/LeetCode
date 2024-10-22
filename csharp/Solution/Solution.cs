using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public long KthLargestLevelSum(TreeNode root, int k)
    {
        List<long> sums = [];
        List<TreeNode> nodes = [root];
        while (nodes.Count > 0)
        {
            sums.Add(nodes.Select(n => (long)n.val).Sum());
            var next = nodes.SelectMany(n => new[] { n.left, n.right }).Where(n => n is not null).ToList();
            nodes = next;
        }
        sums.Sort((a, b) => b.CompareTo(a));
        if (sums.Count >= k) { return sums[k - 1]; }
        else { return -1; }
    }
}

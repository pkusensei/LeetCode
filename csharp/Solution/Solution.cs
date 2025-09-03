using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] FindFrequentTreeSum(TreeNode root)
    {
        Dictionary<int, int> freq = [];
        int f = 0;
        List<int> res = [];
        Dfs(root);
        return [.. res];

        int Dfs(TreeNode node)
        {
            if (node is null) { return 0; }
            int left = Dfs(node.left);
            int right = Dfs(node.right);
            int val = node.val + left + right;
            if (!freq.TryAdd(val, 1)) { freq[val] += 1; }
            if (freq[val] > f)
            {
                f = freq[val];
                res.Clear();
            }
            if (f == freq[val]) { res.Add(val); }
            return val;
        }
    }
}
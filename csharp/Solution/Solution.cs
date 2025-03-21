using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinimumOperations(TreeNode root)
    {
        List<TreeNode> nodes = [root];
        int res = 0;
        while (nodes.Count > 0)
        {
            var nums = nodes.Select(n => n.val).ToList();
            res += Swaps(nums);
            nodes = nodes.SelectMany(n => new[] { n.left, n.right }).Where(n => n is not null).ToList();
        }
        return res;

        static int Swaps(IList<int> nums)
        {
            if (nums.Count < 2) { return 0; }
            var sorted = nums.Select((v, i) => (i, v)).OrderBy(p => p.v).Select(p => p.i).ToList();
            Span<bool> seen = stackalloc bool[nums.Count];
            int res = 0;
            for (int i = 0; i < nums.Count; i++)
            {
                if (seen[i] || sorted[i] == i) { continue; }
                int curr = i;
                int cycle = 0;
                while (!seen[curr])
                {
                    seen[curr] = true;
                    curr = sorted[curr];
                    cycle += 1;
                }
                res += Math.Max(cycle - 1, 0);
            }
            return res;
        }
    }
}

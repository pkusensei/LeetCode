using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestArithSeqLength(int[] nums)
    {
        int n = nums.Length;
        int res = 2;
        List<Dictionary<int, int>> dp = [];
        for (int right = 0; right < n; right++)
        {
            Dictionary<int, int> curr = [];
            for (int left = 0; left < right; left++)
            {
                int d = nums[right] - nums[left];
                curr.TryAdd(d, 2);
                if (dp[left].TryGetValue(d, out var count))
                {
                    curr[d] = int.Max(curr[d], 1 + count);
                    res = int.Max(res, 1 + count);
                }
            }
            dp.Add(curr);
        }
        return res;
    }
}


using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumberOfArithmeticSlices(int[] nums)
    {
        int n = nums.Length;
        int res = 0;
        Dictionary<long, int>[] dp = [.. Enumerable.Range(0, n).Select(_ => new Dictionary<long, int>())];
        for (int right = 1; right < n; right++)
        {
            for (int left = 0; left < right; left++)
            {
                long diff = nums[right] - (long)nums[left];
                int count = dp[left].GetValueOrDefault(diff, 0);
                if (!dp[right].TryAdd(diff, 1 + count)) { dp[right][diff] += 1 + count; }
                res += count;
            }
        }
        return res;
    }
}
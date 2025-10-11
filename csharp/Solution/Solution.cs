using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MaximumTotalDamage(int[] power)
    {
        (long num, int c)[] nums = [.. power.CountBy(x => x).OrderBy(x => x.Key).Select(x => ((long)x.Key, x.Value))];
        int n = nums.Length;
        long[] dp = [.. nums.Select(x => x.num * x.c)];
        for (int left = 0; left < n; left++)
        {
            int right = 1 + left;
            while (right < n && nums[right].num <= nums[left].num + 2)
            {
                right += 1;
            }
            for (int i = 0; i < 3 && right < n; i += 1, right += 1)
            {
                dp[right] = long.Max(dp[right], dp[left] + nums[right].num * nums[right].c);
            }
        }
        return dp.Max();
    }
}

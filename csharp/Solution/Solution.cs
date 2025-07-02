using System.Security.Principal;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PossibleStringCount(string word, int k)
    {
        const long M = 1_000_000_007;
        List<int> nums = [];
        int curr = 1;
        for (int i = 1; i < word.Length; i++)
        {
            if (word[i] != word[i - 1])
            {
                nums.Add(curr);
                curr = 0;
            }
            curr += 1;
        }
        nums.Add(curr);
        long prod = nums.Aggregate(1L, (acc, v) => acc * v % M);
        if (k <= nums.Count) { return (int)prod; }
        long[] dp = new long[k];
        dp[0] = 1;
        for (int idx = 1; idx <= nums.Count; idx++)
        {
            List<long> prefix = [0];
            foreach (var item in dp)
            {
                prefix.Add((item + prefix.Last()) % M);
            }
            dp = new long[k];
            for (int len = idx; len < k; len++)
            {
                int last = len - Math.Min(nums[idx - 1], len + 1 - idx);
                dp[len] = (prefix[len] - prefix[last] + M) % M;
            }
        }
        long lessThanK = dp.Aggregate(0L, (acc, v) => (acc + v) % M);
        return (int)((prod - lessThanK + M) % M);
    }
}

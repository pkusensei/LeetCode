using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int FindTargetSumWays(int[] nums, int target)
    {
        return Dfs(nums, 0, target, []);

        static int Dfs(int[] nums, int idx, int target, Dictionary<(int, int), int> memo)
        {
            if (idx == nums.Length) { return target == 0 ? 1 : 0; }
            if (memo.TryGetValue((idx, target), out var v)) { return v; }
            var curr = nums[idx];
            var res = Dfs(nums, 1 + idx, target + curr, memo) + Dfs(nums, 1 + idx, target - curr, memo);
            memo.Add((idx, target), res);
            return res;
        }
    }

    public int WithDp(int[] nums, int target)
    {
        var total = nums.Sum();
        var dp = new int[1 + 2 * total];
        dp[nums[0] + total] += 1;
        dp[-nums[0] + total] += 1;
        foreach (var (idx, num) in nums.Select((n, i) => (i, n)).Skip(1))
        {
            var next = new int[1 + 2 * total];
            for (int sum = -total; sum <= total; sum++)
            {
                if (dp[sum + total] > 0)
                {
                    next[sum + num + total] += dp[sum + total];
                    next[sum - num + total] += dp[sum + total];
                }
            }
            dp = next;
        }
        return Math.Abs(target) > total ? 0 : dp[target + total];
    }
}
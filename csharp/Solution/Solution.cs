using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxProfit(int[] prices)
    {
        int buy = int.MinValue;
        int sell = 0;
        int cool = 0;
        foreach (var num in prices)
        {
            int b = int.Max(buy, cool - num);
            int s = int.Max(sell, buy + num);
            int c = int.Max(cool, sell);
            (buy, sell, cool) = (b, s, c);
        }
        return sell;
    }

    public int WithDfs(int[] prices)
    {
        int n = prices.Length;
        int[,] memo = new int[n, 2];
        for (int i = 0; i < n; i++)
        {
            memo[i, 0] = -1;
            memo[i, 1] = -1;
        }
        return Dfs(prices, 1);

        int Dfs(ReadOnlySpan<int> nums, int buy)
        {
            if (nums.IsEmpty) { return 0; }
            int n = nums.Length;
            if (memo[n, buy] != -1) { return memo[n, buy]; }
            int res = Dfs(nums[1..], buy);
            if (buy == 1)
            {
                res = int.Max(res, -nums[0] + Dfs(nums[1..], 0));
            }
            else
            {
                res = int.Max(res, nums[0] + Dfs(nums[int.Min(2, nums.Length)..], 1));
            }
            memo[n, buy] = res;
            return res;
        }
    }

    public int CountHillValley(int[] nums)
    {
        List<int> vals = [];
        foreach (var num in nums)
        {
            if (vals.Count > 0 && vals[^1] == num) { continue; }
            vals.Add(num);
        }
        int res = 0;
        for (int i = 1; i < vals.Count - 1; i++)
        {
            if (int.Sign(vals[i] - vals[i - 1]) * int.Sign(vals[1 + i] - vals[i]) < 0)
            {
                res += 1;
            }
        }
        return res;
    }
}
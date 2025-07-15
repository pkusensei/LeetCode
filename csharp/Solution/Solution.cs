using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxProfit(int[] prices)
    {
        int pref = prices[0];
        int res = 0;
        foreach (var item in prices[1..])
        {
            res = int.Max(res, item - pref);
            pref = int.Min(pref, item);
        }
        return res;
    }

    public int MaxProfit2(int[] prices)
    {
        int res = 0;
        for (int i = 0; i < prices.Length - 1; i++)
        {
            res += int.Max(0, prices[1 + i] - prices[i]);
        }
        return res;
    }

    public int MaxProfit3(int[] prices)
    {
        int n = prices.Length;
        if (n < 2) { return 0; }
        int[] left = new int[n];
        int[] right = new int[n];
        int min = prices[0];
        for (int i = 1; i < n; i++)
        {
            min = int.Min(min, prices[i]);
            left[i] = int.Max(left[i - 1], prices[i] - min);
        }
        int max = prices[n - 1];
        for (int i = n - 2; i >= 0; i -= 1)
        {
            max = int.Max(max, prices[i]);
            right[i] = int.Max(right[1 + i], max - prices[i]);
        }
        return left.Zip(right).Select(p => p.First + p.Second).Max();
    }

    public int MaxProfit3With1Pass(int[] prices)
    {
        int cost1 = prices[0];
        int profit1 = 0;
        int cost2 = int.MaxValue;
        int profit2 = 0;
        foreach (var p in prices)
        {
            cost1 = int.Min(cost1, p);
            profit1 = int.Max(profit1, p - cost1);
            cost2 = int.Min(cost2, p - profit1);
            profit2 = int.Max(profit2, p - cost2);
        }
        return profit2;
    }
}
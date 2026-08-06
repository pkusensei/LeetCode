using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Net.Sockets;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int KConcatenationMaxSum(int[] arr, int k)
    {
        const long M = 1_000_000_007;
        int n = arr.Length;
        long curr = 0;
        long total_max = long.MinValue;
        foreach (var item in arr)
        {
            curr = long.Max(item, item + curr);
            total_max = long.Max(total_max, curr);
        }
        long res = long.Max(total_max, 0);
        if (k == 1) { return (int)(res % M); }
        long sum = 0;
        long pref_max = long.MinValue;
        for (int i = 0; i < n; i++)
        {
            sum += arr[i];
            pref_max = long.Max(pref_max, sum);
        }
        sum = 0;
        long suf_max = long.MinValue;
        for (int i = n - 1; i >= 0; i -= 1)
        {
            sum += arr[i];
            suf_max = long.Max(suf_max, sum);
        }
        long v = long.Max(sum * k, sum * (k - 2) + pref_max + suf_max);
        v = long.Max(v, pref_max + suf_max);
        res = long.Max(res, v);
        return (int)(res % M);
    }
}
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string NearestPalindromic(string n)
    {
        int len = n.Length;
        long num = long.Parse(n);
        if (len == 1) { return (num - 1).ToString(); }
        long half = long.Parse(n.AsSpan()[..((1 + len) / 2)]);
        bool odd = (len & 1) == 1;
        List<long> candids = [Build(half, odd), Build(half - 1, odd), Build(1 + half, odd),
            (long)Math.Pow(10, len)+1, (long)Math.Pow(10, len-1)-1];
        long res = long.MaxValue;
        foreach (var item in candids.Where(v => v != num))
        {
            if (long.Abs(item - num) < long.Abs(res - num)) { res = item; }
            if (long.Abs(item - num) == long.Abs(res - num)) { res = long.Min(res, item); }
        }
        return res.ToString();

        static long Build(long half, bool odd)
        {
            long res = half;
            if (odd) { half /= 10; }
            while (half > 0)
            {
                res = 10 * res + half % 10;
                half /= 10;
            }
            return res;
        }
    }
}
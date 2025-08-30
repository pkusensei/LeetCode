using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string SmallestGoodBase(string n)
    {
        // n  = k^0 + k^1 + .. + k^(p-1)
        // n > k^(p-1) ==> k<n^(1/(p-1))
        // kn =       k^1 + .. + k^(p-1) + k^p
        // (k-1)n = k^p - 1
        // n = (k^p - 1)/(k-1) ==> n <= k^p - 1 ==> (n+1)^(1/p)<=k
        if (!long.TryParse(n, out var num)) { return n; }
        int max_p = (int)Math.Log2(1 + num);
        for (int p = max_p; p > 2; p -= 1)
        {
            long left = (long)Math.Pow(1 + num, 1.0 / p);
            long right = (long)Math.Pow(num, 1.0 / (p - 1));
            while (left <= right)
            {
                long mid = left + (right - left) / 2;
                long acc = 0;
                for (int i = 0; i < p; i++)
                {
                    acc = acc * mid + 1;
                }
                if (num == acc) { return mid.ToString(); }
                else if (acc < num) { left = 1 + mid; }
                else { right = mid - 1; }
            }
        }
        return (num - 1).ToString();
    }
}
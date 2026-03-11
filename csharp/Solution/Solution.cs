using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ThreeSumMulti(int[] arr, int target)
    {
        const long M = 1_000_000_007;
        Dictionary<int, long> freq = [];
        foreach (var item in arr)
        {
            if (!freq.TryAdd(item, 1)) { freq[item] += 1; }
        }
        long res = 0;
        foreach (var (n1, f1) in freq)
        {
            foreach (var (n2, f2) in freq)
            {
                int n3 = target - n1 - n2;
                if (freq.TryGetValue(n3, out long f3))
                {
                    if (n1 == n2 && n2 == n3)
                    {
                        res += f1 * (f1 - 1) * (f1 - 2) / 6 % M;
                    }
                    if (n1 == n2 && n2 != n3)
                    {
                        res += f1 * (f1 - 1) / 2 * f3 % M;
                    }
                    if (n1 < n2 && n2 < n3)
                    {
                        res += f1 * f2 * f3 % M;
                    }
                    res %= M;
                }
            }
        }
        return (int)res;
    }
}
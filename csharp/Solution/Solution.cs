using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MinCost(int[] basket1, int[] basket2)
    {
        Dictionary<int, int> freq = [];
        int min_num = int.MaxValue;
        foreach (var num in basket1)
        {
            if (!freq.TryAdd(num, 1)) { freq[num] += 1; }
            min_num = int.Min(min_num, num);
        }
        foreach (var num in basket2)
        {
            if (!freq.TryAdd(num, -1)) { freq[num] -= 1; }
            min_num = int.Min(min_num, num);
        }
        List<int> diff = [];
        foreach (var (num, f) in freq)
        {
            if ((f & 1) == 1) { return -1; }
            if (f != 0) { diff.AddRange(Enumerable.Repeat(num, int.Abs(f) / 2)); }
        }
        diff.Sort();
        long res = 0;
        foreach (var num in diff[..(diff.Count / 2)])
        {
            res += long.Min(num, 2 * min_num);
        }
        return res;
    }
}
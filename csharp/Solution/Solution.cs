using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestWPI(int[] hours)
    {
        int prefix = 0;
        Dictionary<int, int> dict = [];
        int res = 0;
        for (int i = 0; i < hours.Length; i += 1)
        {
            prefix += hours[i] > 8 ? 1 : -1;
            if (prefix > 0) { res = int.Max(res, 1 + i); }
            if (dict.TryGetValue(prefix - 1, out var left)) { res = int.Max(res, 1 + i - left); }
            dict.TryAdd(prefix, i);
        }
        return res;
    }
}

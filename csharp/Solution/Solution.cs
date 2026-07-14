using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumEquivDominoPairs(int[][] dominoes)
    {
        Dictionary<int, int> freq = [];
        foreach (var item in dominoes)
        {
            int a = int.Min(item[0], item[1]);
            int b = int.Max(item[0], item[1]);
            int val = (a << 5) | b;
            if (!freq.TryAdd(val, 1)) { freq[val] += 1; }
        }
        return freq.Values.Sum(v => v * (v - 1) / 2);
    }
}

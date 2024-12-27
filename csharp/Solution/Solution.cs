using System.Collections.Immutable;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxScoreSightseeingPair(int[] values)
    {
        // (values[i]+i) + (values[j]-j)
        var curr = values[0];
        var res = 0;
        foreach (var (idx, val) in values.Select((v, i) => (i, v)).Skip(1))
        {
            res = Math.Max(res, curr + val - idx);
            curr = Math.Max(curr, val + idx);
        }
        return res;
    }
}
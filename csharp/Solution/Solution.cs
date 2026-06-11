using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxScoreSightseeingPair(int[] values)
    {
        int res = 0;
        int prev = values[0] + 0;
        for (int i = 1; i < values.Length; i++)
        {
            res = int.Max(res, prev + values[i] - i);
            prev = int.Max(prev, values[i] + i);
        }
        return res;
    }
}


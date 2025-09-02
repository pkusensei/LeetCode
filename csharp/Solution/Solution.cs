using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindPoisonedDuration(int[] timeSeries, int duration)
    {
        if (duration == 0 || timeSeries.Length == 0) { return 0; }
        int res = 0;
        for (int i = 0; i < timeSeries.Length - 1; i++)
        {
            res += int.Min(timeSeries[1 + i] - timeSeries[i], duration);
        }
        return res + duration;
    }
}

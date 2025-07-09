using System.Reflection.Metadata;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxFreeTime(int eventTime, int k, int[] startTime, int[] endTime)
    {
        List<int> gaps = [];
        int end = 0;
        foreach (var (s, e) in startTime.Zip(endTime))
        {
            gaps.Add(s - end);
            end = e;
        }
        gaps.Add(eventTime - end);
        int res = 0;
        int curr = 0;
        for (int i = 0; i < gaps.Count; i++)
        {
            curr += gaps[i];
            if (i >= 1 + k) { curr -= gaps[i - 1 - k]; }
            res = Math.Max(res, curr);
        }
        return res;
    }
}

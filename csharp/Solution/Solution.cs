using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxFreeTime(int eventTime, int[] startTime, int[] endTime)
    {
        int n = startTime.Length;
        int res = 0;
        int t1 = 0;
        int t2 = 0;
        for (int i = 0; i < n; i++)
        {
            int left1 = i == 0 ? 0 : endTime[i - 1];
            int right1 = i == n - 1 ? eventTime : startTime[i + 1];
            if (endTime[i] - startTime[i] <= t1) { res = Math.Max(res, right1 - left1); }
            t1 = Math.Max(t1, startTime[i] - (i == 0 ? 0 : endTime[i - 1]));
            res = Math.Max(res, right1 - left1 - (endTime[i] - startTime[i]));

            int left2 = i == n - 1 ? 0 : endTime[n - i - 2];
            int right2 = i == 0 ? eventTime : startTime[n - i];
            if (endTime[n - i - 1] - startTime[n - i - 1] <= t2) { res = Math.Max(res, right2 - left2); }
            t2 = Math.Max(t2, (i == 0 ? eventTime : startTime[n - i]) - endTime[n - i - 1]);
        }
        return res;
    }
}

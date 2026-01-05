using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumFriendRequests(int[] ages)
    {
        int n = ages.Length;
        Array.Sort(ages);
        int res = 0;
        int left = 0;
        int right = 0;
        foreach (var num in ages)
        {
            while (left < n && ages[left] <= num / 2 + 7)
            {
                left += 1;
            }
            while (right < n && ages[right] <= num)
            {
                right += 1;
            }
            res += int.Max(0, right - left - 1);
        }
        return res;
    }
}

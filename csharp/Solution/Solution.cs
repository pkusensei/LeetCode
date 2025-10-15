using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxIncreasingSubarrays(IList<int> nums)
    {
        int n = nums.Count;
        int res = 0;
        int left = 0;
        int prev = 0;
        for (int right = 1; right < n; right++)
        {
            if (nums[right - 1] < nums[right]) { continue; }
            int a = (right - left) / 2;
            int b = int.Min(prev, right - left);
            res = int.Max(res, int.Max(a, b));
            prev = right - left;
            left = right;
        }
        int aa = (n - left) / 2;
        int bb = int.Min(prev, n - left);
        res = int.Max(res, int.Max(aa, bb));
        return res;
    }
}

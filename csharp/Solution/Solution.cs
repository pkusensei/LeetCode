using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxWidthRamp(int[] nums)
    {
        int n = nums.Length;
        int[] suf_max = [.. nums];
        for (int i = n - 2; i >= 0; i -= 1)
        {
            suf_max[i] = int.Max(suf_max[i], suf_max[1 + i]);
        }
        int res = 0;
        int left = 0;
        int right = 0;
        while (right < n)
        {
            while (left < right && nums[left] > suf_max[right])
            {
                left += 1;
            }
            res = int.Max(res, right - left);
            right += 1;
        }
        return res;
    }
}

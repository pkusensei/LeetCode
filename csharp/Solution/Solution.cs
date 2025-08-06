using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumberOfArithmeticSlices(int[] nums)
    {
        int n = nums.Length;
        int res = 0;
        for (int left = 0; left < n; left++)
        {
            int right = 1 + left;
            if (right >= n) { break; }
            int d = nums[right] - nums[left];
            for (; 1 + right < n && nums[1 + right] - nums[right] == d; right++) { }
            int len = right - left;
            res += len * (len - 1) / 2;
            left = right - 1;
        }
        return res;
    }
}

using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long ZeroFilledSubarray(int[] nums)
    {
        long res = 0;
        for (int left = 0; left < nums.Length; left++)
        {
            if (nums[left] == 0)
            {
                int right = left;
                for (; right < nums.Length && nums[right] == 0; right++) { }
                long len = right - left;
                res += len * (1 + len) / 2;
                left = right - 1;
            }
        }
        return res;
    }
}
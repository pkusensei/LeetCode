using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long CountSubarrays(int[] nums, long k)
    {
        int left = 0;
        long window = 0;
        long res = 0;
        for (int right = 0; right < nums.Length; right++)
        {
            window += nums[right];
            while (window * (right + 1 - left) >= k)
            {
                window -= nums[left];
                left += 1;
            }
            res += right + 1 - left;
        }
        return res;
    }
}

using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long CountSubarrays(int[] nums, int k)
    {
        int max = nums.Max();
        int left = 0;
        long res = 0;
        int count = 0;
        for (int right = 0; right < nums.Length; right += 1)
        {
            count += nums[right] == max ? 1 : 0;
            while (count > k)
            {
                count -= nums[left] == max ? 1 : 0;
                left += 1;
            }
            res += right + 1 - left;
        }
        long n = nums.Length;
        return n * (1 + n) / 2 - res;
    }
}


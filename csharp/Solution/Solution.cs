using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumSubseq(int[] nums, int target)
    {
        const int M = 1_000_000_007;
        Array.Sort(nums);
        int[] pow = new int[nums.Length];
        pow[0] = 1;
        for (int i = 1; i < nums.Length; i++)
        {
            pow[i] = 2 * pow[i - 1] % M;
        }
        int left = 0;
        int right = nums.Length - 1;
        int res = 0;
        while (left <= right)
        {
            if (nums[left] + nums[right] > target) { right -= 1; }
            else
            {
                res = (res + pow[right - left]) % M;
                left += 1;
            }
        }
        return res;
    }
}

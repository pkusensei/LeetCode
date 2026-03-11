using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] SortArrayByParityII(int[] nums)
    {
        int n = nums.Length;
        int even_i = 0;
        int odd_i = 1;
        while (even_i < n && odd_i < n)
        {
            if ((nums[even_i] & 1) == 0)
            {
                even_i += 2;
                continue;
            }
            if ((nums[odd_i] & 1) == 1)
            {
                odd_i += 2;
                continue;
            }
            (nums[even_i], nums[odd_i]) = (nums[odd_i], nums[even_i]);
        }
        return nums;
    }
}
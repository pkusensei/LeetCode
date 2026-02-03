using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsTrionic(int[] nums)
    {
        int prev = -1;
        int res = 0;
        foreach (var item in nums.Zip(nums.Skip(1)).Select(v => v.Second - v.First))
        {
            int curr = item switch
            {
                > 0 => 1,
                < 0 => -1,
                0 => 0
            };
            if (curr == 0) { return false; }
            if (prev != curr)
            {
                res += 1;
                prev = curr;
            }
        }
        return res == 3 && nums[0] < nums[1];
    }
}
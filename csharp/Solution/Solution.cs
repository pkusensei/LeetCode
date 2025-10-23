using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool HasSameDigits(string s)
    {
        List<int> nums = [.. s.Select(c => c - 'a')];
        while (nums.Count > 2)
        {
            List<int> curr = new(nums.Count - 1);
            for (int i = 0; i < nums.Count - 1; i++)
            {
                curr.Add((nums[i] + nums[1 + i]) % 10);
            }
            nums = curr;
        }
        return nums[0] == nums[1];
    }
}
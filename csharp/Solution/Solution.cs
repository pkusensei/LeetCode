using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinimumIndex(IList<int> nums)
    {
        int major = nums[0];
        int count = 0;
        foreach (var item in nums)
        {
            if (major == item) { count += 1; }
            else { count -= 1; }
            if (count == 0)
            {
                major = item;
                count = 1;
            }
        }
        int left = 0;
        int right = nums.Where(v => v == major).Count();
        for (int i = 0; i < nums.Count; i++)
        {
            left += nums[i] == major ? 1 : 0;
            right -= nums[i] == major ? 1 : 0;
            if (left > (1 + i) / 2 && right > (nums.Count - i - 1) / 2) { return i; }
        }
        return -1;
    }
}

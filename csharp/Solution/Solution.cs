using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxDistinctElements(int[] nums, int k)
    {
        int n = nums.Length;
        Array.Sort(nums);
        int curr = nums[0] - k;
        int res = 1;
        for (int i = 0; i < n; i++)
        {
            if (curr > nums[i] + k) { continue; }
            nums[i] = int.Max(curr, nums[i] - k);
            curr = 1 + nums[i];
            if (i > 0 && nums[i] > nums[i - 1]) { res += 1; }
        }
        return res;
    }
}

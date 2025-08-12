using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FindDuplicates(int[] nums)
    {
        List<int> res = [];
        for (int i = 0; i < nums.Length; i++)
        {
            int val = int.Abs(nums[i]);
            int target = nums[val - 1];
            if (target < 0) { res.Add(val); }
            nums[val - 1] = -int.Abs(nums[val - 1]);
        }
        return res;
    }
}
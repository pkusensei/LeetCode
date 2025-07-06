using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> ThreeSum(int[] nums)
    {
        Array.Sort(nums);
        List<IList<int>> res = [];
        for (int i = 0; i < nums.Length; i++)
        {
            if (nums[i] > 0) { break; }
            if (i > 0 && nums[i] == nums[i - 1]) { continue; }
            int left = 1 + i;
            int right = nums.Length - 1;
            while (left < right)
            {
                int sum = nums[i] + nums[left] + nums[right];
                if (sum < 0) { left += 1; }
                else if (sum > 0) { right -= 1; }
                else
                {
                    res.Add([nums[i], nums[left], nums[right]]);
                    left += 1;
                    while (left < right && nums[left] == nums[left - 1]) { left += 1; }
                }
            }
        }
        return res;
    }

    public int ThreeSumClosest(int[] nums, int target)
    {
        Array.Sort(nums);
        int res = nums[..3].Sum();
        for (int i = 0; i < nums.Length - 2; i++)
        {
            int left = 1 + i;
            int right = nums.Length - 1;
            while (left < right)
            {
                int curr = nums[i] + nums[left] + nums[right];
                if (Math.Abs(curr - target) < Math.Abs(res - target)) { res = curr; }
                if (curr > target) { right -= 1; }
                else if (curr < target) { left += 1; }
                else { return res; }
            }
        }
        return res;
    }

    public IList<IList<int>> FourSum(int[] nums, int target)
    {
        Array.Sort(nums);
        return XSum(nums, target, 4);

        static List<IList<int>> XSum(Span<int> nums, long target, int k)
        {
            List<IList<int>> res = [];
            if (nums.Length < 1) { return res; }
            if (k == 2) { return TwoSum(nums, target); }
            for (int i = 0; i < nums.Length; i++)
            {
                if (i == 0 || nums[i - 1] != nums[i])
                {
                    foreach (var item in XSum(nums[(1 + i)..], target - nums[i], k - 1))
                    {
                        res.Add([nums[i], .. item]);
                    }
                }
            }
            return res;
        }

        static List<IList<int>> TwoSum(Span<int> nums, long target)
        {
            List<IList<int>> res = [];
            int left = 0;
            int right = nums.Length - 1;
            while (left < right)
            {
                long curr = nums[left] + nums[right];
                if (curr < target || (left > 0 && nums[left] == nums[left - 1])) { left += 1; }
                else if (curr > target) { right -= 1; }
                else
                {
                    res.Add([nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                }
            }
            return res;
        }
    }
}

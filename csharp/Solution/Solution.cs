using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ReversePairs(int[] nums)
    {
        return MergeSort(nums);

        static int MergeSort(Span<int> nums)
        {
            if (nums.Length < 2) { return 0; }
            int mid = nums.Length / 2;
            int res = MergeSort(nums[..mid]) + MergeSort(nums[mid..]);
            int left = 0;
            int right = mid;
            for (; left < mid; left++)
            {
                for (; right < nums.Length && nums[left] > 2L * nums[right]; right++) { }
                res += right - mid;
            }
            left = 0;
            right = mid;
            List<int> vals = new(nums.Length);
            while (left < mid && right < nums.Length)
            {
                if (nums[left] <= nums[right])
                {
                    vals.Add(nums[left]);
                    left += 1;
                }
                else
                {
                    vals.Add(nums[right]);
                    right += 1;
                }
            }
            vals.AddRange(nums[left..mid]);
            vals.AddRange(nums[right..]);
            for (int i = 0; i < nums.Length; i++)
            {
                nums[i] = vals[i];
            }
            return res;
        }
    }
}

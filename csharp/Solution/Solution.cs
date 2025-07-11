using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int RemoveDuplicates(int[] nums)
    {
        int n = nums.Length;
        if (n <= 2) { return n; }
        int left = 2;
        for (int right = 2; right < n; right++)
        {
            if (nums[right] != nums[left - 2])
            {
                nums[left] = nums[right];
                left += 1;
            }
        }
        return left;
    }
}
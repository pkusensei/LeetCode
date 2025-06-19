using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PartitionArray(int[] nums, int k)
    {
        Array.Sort(nums);
        int res = 1;
        int left = 0;
        for (int i = 0; i < nums.Length; i++)
        {
            if (nums[i] - nums[left] > k)
            {
                res += 1;
                left = i;
            }
        }
        return res;
    }
}

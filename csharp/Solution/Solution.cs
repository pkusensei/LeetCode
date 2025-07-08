using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int RemoveDuplicates(int[] nums)
    {
        int curr = 0;
        for (int i = 0; i < nums.Length; i++)
        {
            if (i < nums.Length - 1 && nums[i] == nums[1 + i]) { continue; }
            nums[curr] = nums[i];
            curr += 1;
        }
        return curr;
    }
}

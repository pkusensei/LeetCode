using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int RemoveElement(int[] nums, int val)
    {
        int curr = 0;
        for (int i = 0; i < nums.Length; i++)
        {
            if (nums[i] == val) { continue; }
            nums[curr] = nums[i];
            curr += 1;
        }
        return curr;
    }
}

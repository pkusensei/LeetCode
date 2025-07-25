using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void MoveZeroes(int[] nums)
    {
        for (int slow = 0; slow < nums.Length; slow++)
        {
            if (nums[slow] == 0)
            {
                for (int fast = 1 + slow; fast < nums.Length; fast++)
                {
                    if (nums[fast] != 0)
                    {
                        (nums[slow], nums[fast]) = (nums[fast], nums[slow]);
                        break;
                    }
                }
            }
        }
    }
}
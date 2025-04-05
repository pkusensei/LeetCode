using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int SubsetXORSum(int[] nums)
    {
        int res = 0;
        Backtrack(nums, 0);
        return res;

        void Backtrack(Span<int> nums, int curr)
        {
            if (nums.Length == 0) { res += curr; }
            else
            {
                Backtrack(nums[1..], curr);
                Backtrack(nums[1..], curr ^ nums[0]);
            }
        }
    }

    public int WithBits(int[] nums)
    {
        int bitor = nums.Aggregate(0, (a, b) => a | b);
        return bitor * (1 << nums.Length - 1);
    }
}

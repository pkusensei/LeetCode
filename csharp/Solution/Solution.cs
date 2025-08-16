using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinMoves(int[] nums)
    {
        int min = nums.Min();
        int res = 0;
        foreach (var num in nums)
        {
            res += num - min;
        }
        return res;
    }
}
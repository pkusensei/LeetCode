using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinPatches(int[] nums, int n)
    {
        long curr = 0;
        int res = 0;
        int idx = 0;
        while (curr < n)
        {
            if (idx < nums.Length && nums[idx] <= 1 + curr)
            {
                curr += nums[idx]; // try reach next number
                idx += 1;
            }
            else
            {
                curr = 1 + 2 * curr; // if not, jump by/fill in (1+curr)
                res += 1;
            }
        }
        return res;
    }
}

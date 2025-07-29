using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] SmallestSubarrays(int[] nums)
    {
        Span<int> next = stackalloc int[32];
        int[] res = new int[nums.Length];
        for (int i = nums.Length - 1; i >= 0; i -= 1)
        {
            res[i] = 1;
            for (int bit = 0; bit < 32; bit++)
            {
                if (((nums[i] >> bit) & 1) == 1) { next[bit] = i; }
                res[i] = int.Max(res[i], next[bit] + 1 - i);
            }
        }
        return res;
    }
}

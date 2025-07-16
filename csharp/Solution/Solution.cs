using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximumLength(int[] nums)
    {
        int even = 0;
        int odd = 0;
        int even_odd = 0;
        int odd_even = 0;
        foreach (var num in nums)
        {
            int next_even_odd = even_odd;
            int next_odd_even = odd_even;
            if ((num & 1) == 0)
            {
                even += 1;
                next_odd_even = 1 + even_odd;
            }
            else
            {
                odd += 1;
                next_even_odd = 1 + odd_even;
            }
            even_odd = next_even_odd;
            odd_even = next_odd_even;
        }
        return int.Max(int.Max(even, odd), int.Max(even_odd, odd_even));
    }
}
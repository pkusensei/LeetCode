using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SingleNumber(int[] nums)
    {
        int one = 0;
        int two = 0;
        foreach (var num in nums)
        {
            for (int bit = 0; bit < 32; bit++)
            {
                if (((num >> bit) & 1) == 1)
                {
                    if (((two >> bit) & 1) == 1)
                    {
                        two ^= 1 << bit;
                    }
                    else if (((one >> bit) & 1) == 1)
                    {
                        one ^= 1 << bit;
                        two |= 1 << bit;
                    }
                    else
                    {
                        one |= 1 << bit;
                    }
                }
            }
        }
        return one;
    }
}
using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsOneBitCharacter(int[] bits)
    {
        bool prev_zero = false;
        for (int i = 0; i < bits.Length; i++)
        {
            if (bits[i] == 0)
            {
                prev_zero = true;
            }
            else
            {
                prev_zero = false;
                i += 1;
            }
        }
        return prev_zero;
    }
}


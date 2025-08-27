using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindComplement(int num)
    {
        int zeros = int.LeadingZeroCount(num);
        return ((1 << (32 - zeros)) - 1) ^ num;
    }
}
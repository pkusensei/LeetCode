using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int GetSum(int a, int b)
    {
        while (b != 0)
        {
            int sum_no_carry = a ^ b;
            int carry = a & b;
            a = sum_no_carry;
            b = carry << 1;
        }
        return a;
    }
}
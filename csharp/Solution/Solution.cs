using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MakeTheIntegerZero(int num1, int num2)
    {
        for (int i = 0; i <= 60; i++)
        {
            long val = (long)num1 - (long)i * num2;
            if (i <= val && long.PopCount(val) <= i) { return i; }
        }
        return -1;
    }
}

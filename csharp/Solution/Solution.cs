using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] GetNoZeroIntegers(int n)
    {
        for (int i = 1; i <= n / 2; i++)
        {
            if (Check(i) && Check(n - i)) { return [i, n - i]; }
        }
        return [];

        static bool Check(int num)
        {
            while (num > 0)
            {
                if (num % 10 == 0) { return false; }
                num /= 10;
            }
            return true;
        }
    }
}

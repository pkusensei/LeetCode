using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Rand10()
    {
        while (true)
        {
            int a = Rand7();
            int b = Rand7();
            int c = (a - 1) * 7 + (b - 1);
            if (c < 40) { return c % 10 + 1; }
        }
    }

    int Rand7()
    {
        return 0;
    }
}
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int IntegerReplacement(int n)
    {
        int res = 0;
        long val = n;
        while (val != 1)
        {
            if ((val & 1) == 0) { val /= 2; }
            else if (val == 3 || (val - 1) % 4 == 0) { val -= 1; }
            else { val += 1; }
            res += 1;
        }
        return res;
    }
}

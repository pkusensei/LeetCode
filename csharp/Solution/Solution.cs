using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string BaseNeg2(int n)
    {
        if (n == 0) { return "0"; }
        List<char> res = [];
        while (n != 0)
        {
            int rem = n % -2;
            n /= -2;
            if (rem < 0)
            {
                rem += 2;
                n += 1;
            }
            res.Add(rem == 0 ? '0' : '1');
        }
        res.Reverse();
        return new([.. res]);
    }
}


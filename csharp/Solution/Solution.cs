using System.Security.Principal;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PossibleStringCount(string word)
    {
        char prev = ' ';
        int res = 1;
        int curr = 0;
        foreach (var item in word)
        {
            if (item != prev)
            {
                res += Math.Max(0, curr - 1);
                curr = 0;
            }
            prev = item;
            curr += 1;
        }
        return res + curr - 1;
    }
}

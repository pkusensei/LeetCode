using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinAddToMakeValid(string s)
    {
        int open = 0;
        int res = 0;
        foreach (var item in s)
        {
            if (item == '(') { open += 1; }
            else { open -= 1; }
            if (open < 0)
            {
                open = 0;
                res += 1;
            }
        }
        return res + open;
    }
}
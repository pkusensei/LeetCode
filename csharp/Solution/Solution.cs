using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinAddToMakeValid(string s)
    {
        var open = 0;
        var res = 0;
        foreach (var item in s)
        {
            if (item == '(') { open += 1; }
            else if (item == ')' && open > 0) { open -= 1; }
            else { res += 1; }
        }
        return res + open;
    }
}

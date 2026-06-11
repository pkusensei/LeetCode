using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string RemoveOuterParentheses(string s)
    {
        int open = 0;
        int left = 0;
        StringBuilder sb = new();
        for (int i = 0; i < s.Length; i++)
        {
            open += s[i] == '(' ? 1 : -1;
            if (open == 0)
            {
                sb.Append(s[(1 + left)..i]);
                left = 1 + i;
            }
        }
        return sb.ToString();
    }
}


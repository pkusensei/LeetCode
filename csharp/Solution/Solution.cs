using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string LastSubstring(string s)
    {
        int n = s.Length;
        int i1 = 0;
        int i2 = 1;
        int len = 0;
        while (i2 + len < n)
        {
            if (s[i1 + len] == s[i2 + len])
            {
                len += 1;
                continue;
            }
            if (s[i1 + len] < s[i2 + len])
            {
                i1 = int.Max(i1 + 1 + len, i2);
                i2 = 1 + i1;
            }
            else
            {
                i2 += 1 + len;
            }
            len = 0;
        }
        return s[i1..];
    }
}

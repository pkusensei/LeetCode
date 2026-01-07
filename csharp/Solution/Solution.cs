using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int UniqueLetterString(string s)
    {
        Span<(int prev1, int prev2)> inds = stackalloc (int, int)[26];
        for (int i = 0; i < 26; i++)
        {
            inds[i] = (-1, -1);
        }
        int n = s.Length;
        int res = 0;
        for (int i = 0; i < n; i++)
        {
            int c = s[i] - 'A';
            res += (i - inds[c].prev2) * (inds[c].prev2 - inds[c].prev1);
            inds[c] = (inds[c].prev2, i);
        }
        for (int i = 0; i < 26; i++)
        {
            res += (n - inds[i].prev2) * (inds[i].prev2 - inds[i].prev1);
        }
        return res;
    }
}

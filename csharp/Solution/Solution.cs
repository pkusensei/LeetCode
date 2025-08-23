using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool RepeatedSubstringPattern(string s)
    {
        int n = s.Length;
        int[] lps = new int[n];
        int len = 0;
        for (int i = 1; i < n; i++)
        {
            while (len > 0 && s[len] != s[i])
            {
                len = lps[len - 1];
            }
            if (s[i] == s[len]) { len += 1; }
            lps[i] = len;
        }
        int val = lps[n - 1];
        return val > 0 && n % (n - val) == 0;
    }
}
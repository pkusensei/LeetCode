using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string DecodeAtIndex(string s, int k)
    {
        long len = 0;
        foreach (var c in s)
        {
            if (char.IsAsciiDigit(c)) { len *= c - '0'; }
            else { len += 1; }
        }
        for (int i = s.Length - 1; i >= 0; i -= 1)
        {
            k = (int)(k % len);
            if (k == 0 && char.IsAsciiLetter(s[i])) { return s[i].ToString(); }
            if (char.IsAsciiDigit(s[i])) { len /= s[i] - '0'; }
            else { len -= 1; }
        }
        return "";
    }
}
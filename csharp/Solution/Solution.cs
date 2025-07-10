using System.Text;
using System.Text.RegularExpressions;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsNumber(string s)
    {
        s = s.Trim();
        int n = s.Length;
        char prev = '#';
        bool seen_e = false;
        bool seen_dot = false;
        bool seen_sign = false;
        for (int i = 0; i < n; i++)
        {
            switch (s[i])
            {
                case char c when '0' <= c && c <= '9': prev = c; break;
                case '+':
                case '-':
                    if ((prev != 'e' && prev != '#') || seen_sign || i == n - 1) { return false; }
                    prev = s[i];
                    seen_sign = true;
                    break;
                case 'e':
                case 'E':
                    if (seen_e || !char.IsAsciiDigit(prev) || i == n - 1) { return false; }
                    prev = char.ToLower(s[i]);
                    seen_e = true;
                    seen_sign = false;
                    break;
                case '.':
                    if (seen_dot || seen_e) { return false; }
                    if (i == 0 && !(i < n - 1 && char.IsAsciiDigit(s[1 + i]))) { return false; }
                    if (prev != '#' && !char.IsAsciiDigit(prev) && prev != '+' && prev != '-') { return false; }
                    if ((prev == '+' || prev == '-') && !(i < n - 1 && char.IsAsciiDigit(s[1 + i]))) { return false; }
                    seen_dot = true;
                    seen_sign = false;
                    break;
                default: return false;
            }
        }
        return true;
    }
}
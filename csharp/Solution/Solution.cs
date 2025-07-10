using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsNumber(string s)
    {
        int exp_idx = s.LastIndexOfAny(['e', 'E']);
        if (exp_idx == 0) { return false; }
        if (exp_idx > 0 && s[(1 + exp_idx)..].Any(c => !char.IsAsciiDigit(c))) { return false; }

        string left = exp_idx > 0 ? s[..exp_idx] : s;
        int dot_idx = left.LastIndexOf('.');
        int start_idx = 0;
        while (left[start_idx] == '+' || left[start_idx] == '-')
        {
            start_idx += 1;
        }
        if (start_idx > 1) { return false; }
        if (dot_idx < 0 && left[start_idx..].Any(c => !char.IsAsciiDigit(c))) { return false; }
        if (dot_idx >= 0)
        {
            if (left.Length < 2) { return false; }
            if (dot_idx > start_idx && left[start_idx..dot_idx].Any(c => !char.IsAsciiDigit(c)))
            {
                return false;
            }
            if (left[(1 + dot_idx)..].Length > 0 && left[(1 + dot_idx)..].Any(c => !char.IsAsciiDigit(c)))
            {
                return false;
            }
        }
        return true;
    }
}
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string FractionToDecimal(int numerator, int denominator)
    {
        if (numerator == 0) { return "0"; }
        StringBuilder sb = new();
        if (numerator < 0 ^ denominator < 0) { sb.Append('-'); }
        long num = long.Abs(numerator);
        long den = long.Abs(denominator);
        sb.Append(num / den);
        num %= den;
        if (num == 0) { return sb.ToString(); }
        sb.Append('.');
        Dictionary<long, int> seen = [];
        while (num > 0)
        {
            if (seen.TryGetValue(num, out var idx))
            {
                sb.Insert(idx, '(');
                sb.Append(')');
                break;
            }
            seen.Add(num, sb.Length);
            num *= 10;
            sb.Append(num / den);
            num %= den;
        }
        return sb.ToString();
    }
}

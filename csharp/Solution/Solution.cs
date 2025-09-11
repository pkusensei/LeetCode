using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string FractionAddition(string expression)
    {
        long nom = 0;
        long den = 1;
        long sign = 1;
        long curr_nom = 0;
        long curr_den = 0;
        bool is_nom = true;
        foreach (var ch in expression)
        {
            switch (ch)
            {
                case '+':
                case '-':
                    curr_nom *= sign;
                    nom = nom * curr_den + curr_nom * den;
                    den *= curr_den;
                    if (den == 0) { den = 1; }
                    long gcd = GCD(long.Abs(nom), den);
                    nom /= gcd;
                    den /= gcd;
                    sign = ch == '-' ? -1 : 1;
                    curr_nom = 0;
                    curr_den = 0;
                    is_nom = true;
                    break;
                case '/':
                    is_nom = false;
                    break;
                case char c when char.IsAsciiDigit(ch):
                    if (is_nom) { curr_nom = 10 * curr_nom + ch - '0'; }
                    else { curr_den = 10 * curr_den + ch - '0'; }
                    break;
                default:
                    break;
            }
        }
        curr_nom *= sign;
        nom = nom * curr_den + curr_nom * den;
        den *= curr_den;
        if (den == 0) { den = 1; }
        long gcd_ = GCD(long.Abs(nom), den);
        nom /= gcd_;
        den /= gcd_;
        return $"{nom}/{den}";

        static long GCD(long a, long b) => a == 0 ? b : GCD(b % a, a);
    }
}

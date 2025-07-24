using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    static readonly string[] DIGITS = ["Zero", "One", "Two", "Three", "Four",
                                       "Five", "Six", "Seven", "Eight", "Nine",
                                       "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen",
                                       "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
    static readonly string[] TENS = ["Ten", "Twenty", "Thirty", "Forty",
                                     "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
    static readonly string[] ORDERS = ["Thousand", "Million", "Billion"];

    public string NumberToWords(int num)
    {
        if (num == 0) { return DIGITS[0]; }
        int order = 0;
        List<string> res = [];
        while (num > 0)
        {
            int curr = num % 1000;
            if (curr > 0) { res.Add(Solve(curr, order)); }
            order += 1;
            num /= 1000;
        }
        return string.Join(' ', res.AsEnumerable().Reverse().ToArray());

        static string Solve(int num, int order)
        {
            if (num == 0) { return string.Empty; }
            StringBuilder sb = new();
            int hun = num / 100;
            if (hun > 0)
            {
                sb.Append(DIGITS[hun]);
                sb.Append(" Hundred");
            }
            int tens = num % 100;
            if (1 <= tens && tens <= 19)
            {
                if (sb.Length > 0) { sb.Append(' '); }
                sb.Append(DIGITS[tens]);
            }
            else if (tens > 0)
            {
                if (sb.Length > 0) { sb.Append(' '); }
                int single = tens % 10;
                tens /= 10;
                sb.Append(TENS[tens - 1]);
                if (single > 0)
                {
                    sb.Append(' ');
                    sb.Append(DIGITS[single]);
                }
            }
            if (order > 0)
            {
                sb.Append(' ');
                sb.Append(ORDERS[order - 1]);
            }
            return sb.ToString();
        }
    }
}
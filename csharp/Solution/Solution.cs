using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> AddOperators(string num, int target)
    {
        List<string> res = [];
        List<string> expr = [];
        Backtrack(num, 0, 0);
        return res;

        void Backtrack(ReadOnlySpan<char> s, long value, long prev)
        {
            if (s.IsEmpty)
            {
                if (value == target) { res.Add(string.Join(string.Empty, expr)); }
                return;
            }
            for (int i = 0; i < s.Length; i++)
            {
                if (s[0] == '0' && i > 0) { return; } // leading zero
                long curr = long.Parse(s[..(1 + i)]);
                string curr_str = new(s[..(1 + i)]);
                if (s.Length == num.Length)
                {
                    expr.Add(curr_str);
                    Backtrack(s[(1 + i)..], curr, curr);
                    expr.RemoveAt(expr.Count - 1);
                }
                else
                {
                    expr.Add("+");
                    expr.Add(curr_str);
                    Backtrack(s[(1 + i)..], value + curr, curr);
                    expr.RemoveAt(expr.Count - 1);
                    expr.RemoveAt(expr.Count - 1);

                    expr.Add("-");
                    expr.Add(curr_str);
                    Backtrack(s[(1 + i)..], value - curr, -curr);
                    expr.RemoveAt(expr.Count - 1);
                    expr.RemoveAt(expr.Count - 1);

                    expr.Add("*");
                    expr.Add(curr_str);
                    Backtrack(s[(1 + i)..], value - prev + prev * curr, prev * curr);
                    expr.RemoveAt(expr.Count - 1);
                    expr.RemoveAt(expr.Count - 1);
                }
            }
        }
    }
}
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> RemoveInvalidParentheses(string s)
    {
        HashSet<string> res = [];
        StringBuilder sb = new(s.Length);
        int max_skip = s.Length;
        Backtrack(s, 0, 0);
        return [.. res];

        void Backtrack(ReadOnlySpan<char> s, int open, int skip)
        {
            if (s.IsEmpty)
            {
                if (open == 0)
                {
                    if (max_skip > skip)
                    {
                        max_skip = skip;
                        res.Clear();
                    }
                    if (skip == max_skip) { res.Add(sb.ToString()); }
                }
                return;
            }
            if (open < 0 || max_skip < skip) { return; }
            switch (s[0])
            {
                case '(':
                    Backtrack(s[1..], open, 1 + skip);
                    AddRemove(s, 1 + open, skip);
                    break;
                case ')':
                    Backtrack(s[1..], open, 1 + skip);
                    AddRemove(s, open - 1, skip);
                    break;
                default:
                    AddRemove(s, open, skip);
                    break;
            }

        }

        void AddRemove(ReadOnlySpan<char> s, int open, int skip)
        {
            sb.Append(s[0]);
            Backtrack(s[1..], open, skip);
            sb.Remove(sb.Length - 1, 1);
        }
    }
}
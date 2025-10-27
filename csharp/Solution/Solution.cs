using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string CountOfAtoms(string formula)
    {
        var res = Parse(formula, 0).dict;
        StringBuilder sb = new();
        foreach (var kv in res.OrderBy(kv => kv.Key))
        {
            sb.Append(kv.Key);
            if (kv.Value > 1) { sb.Append(kv.Value); }
        }
        return sb.ToString();

        static (int end, Dictionary<string, int> dict) Parse(ReadOnlySpan<char> s, int idx)
        {
            Dictionary<string, int> res = [];
            var lookup = res.GetAlternateLookup<ReadOnlySpan<char>>();
            int prev = idx;
            bool done = false;
            while (!done && idx < s.Length)
            {
                switch (s[idx])
                {
                    case char ch when char.IsAsciiLetterUpper(ch):
                        idx += 1;
                        while (idx < s.Length && char.IsAsciiLetterLower(s[idx]))
                        {
                            idx += 1;
                        }
                        var name = s[prev..idx];
                        var temp = Count(s, idx);
                        idx = temp.i;
                        if (!lookup.TryAdd(name, temp.c)) { lookup[name] += temp.c; }
                        prev = idx;
                        break;
                    case '(':
                        var (end, dict) = Parse(s, 1 + idx);
                        idx = end;
                        temp = Count(s, idx);
                        idx = temp.i;
                        prev = idx;
                        foreach (var (k, v) in dict)
                        {
                            if (!lookup.TryAdd(k, temp.c * v)) { lookup[k] += temp.c * v; }
                        }
                        break;
                    default:
                        done = true;
                        idx += 1; // skip ')'
                        break; // do not break while loop
                }
            }
            return (idx, res);
        }

        static (int c, int i) Count(ReadOnlySpan<char> s, int idx)
        {
            int count = 0;
            while (idx < s.Length && char.IsAsciiDigit(s[idx]))
            {
                count = 10 * count + s[idx] - '0';
                idx += 1;
            }
            if (count == 0) { count = 1; }
            return (count, idx);
        }
    }
}
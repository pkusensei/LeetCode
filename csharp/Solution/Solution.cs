using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> DiffWaysToCompute(string expression)
    {
        Dictionary<string, List<int>> memo = [];
        var lookup = memo.GetAlternateLookup<ReadOnlySpan<char>>();
        return Dfs(expression);

        List<int> Dfs(ReadOnlySpan<char> s)
        {
            List<int> res;
            if (lookup.TryGetValue(s, out res)) { return res; }
            res = [];
            if (int.TryParse(s, out var val))
            {
                res.Add(val);
                lookup.TryAdd(s, res);
                return res;
            }
            for (int i = 0; i < s.Length; i++)
            {
                if ("+-*".Contains(s[i]))
                {
                    var left = Dfs(s[..i]);
                    var right = Dfs(s[(1 + i)..]);
                    foreach (var (a, b) in left.SelectMany(a => right.Select(b => (a, b))))
                    {
                        switch (s[i])
                        {
                            case '+': res.Add(a + b); break;
                            case '-': res.Add(a - b); break;
                            case '*': res.Add(a * b); break;
                            default: break;
                        }
                    }
                }
            }
            lookup.TryAdd(s, res);
            return res;
        }
    }
}
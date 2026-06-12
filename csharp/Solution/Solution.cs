using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<bool> CamelMatch(string[] queries, string pattern)
    {
        List<bool> res = new(queries.Length);
        foreach (var q in queries)
        {
            int i = 0;
            bool flag = true;
            foreach (var c in q)
            {
                if (char.IsAsciiLetterUpper(c))
                {
                    if (i < pattern.Length && pattern[i] == c)
                    {
                        i += 1;
                        continue;
                    }
                    else
                    {
                        flag = false;
                        break;
                    }
                }
                else
                {
                    if (i < pattern.Length && pattern[i] == c)
                    {
                        i += 1;
                        continue;
                    }
                }
            }
            res.Add(flag && i == pattern.Length);
        }
        return res;
    }
}


using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<bool> CanMakePaliQueries(string s, int[][] queries)
    {
        int[] curr = new int[26];
        List<int[]> prefix = new(s.Length);
        foreach (var c in s)
        {
            curr[c - 'a'] += 1;
            prefix.Add([.. curr]);
        }
        List<bool> res = new(queries.Length);
        foreach (var q in queries)
        {
            int odd = 0;
            for (int i = 0; i < 26; i++)
            {
                int val = prefix[q[1]][i] - (q[0] > 0 ? prefix[q[0] - 1][i] : 0);
                odd += val & 1;
            }
            res.Add(odd <= 1 + 2 * q[2]);
        }
        return res;
    }
}


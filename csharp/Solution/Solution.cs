using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountDigitOne(int n)
    {
        string s = n.ToString();
        int len = s.Length;
        Dictionary<(int, bool, int), int> memo = [];
        return Dfs(0, true, 0);

        int Dfs(int idx, bool tight, int count)
        {
            if (idx >= len) { return count; }
            int res;
            if (memo.TryGetValue((idx, tight, count), out res)) { return res; }
            int max_d = tight ? s[idx] - '0' : 9;
            for (int d = 0; d <= max_d; d++)
            {
                bool ntight = tight && d == max_d;
                if (d == 1) { res += Dfs(1 + idx, ntight, 1 + count); }
                else { res += Dfs(1 + idx, ntight, count); }
            }
            memo.Add((idx, tight, count), res);
            return res;
        }
    }
}
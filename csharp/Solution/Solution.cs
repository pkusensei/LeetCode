using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxPartitionsAfterOperations(string s, int k)
    {
        Dictionary<(int, int, bool), int> memo = [];
        return Dfs(0, 0, false);

        int Dfs(int idx, int mask, bool change)
        {
            if (idx >= s.Length) { return 1; }
            var key = (idx, mask, change);
            int res;
            if (memo.TryGetValue(key, out res)) { return res; }
            int bit = 1 << (s[idx] - 'a');
            if (int.PopCount(mask | bit) <= k) { res = Dfs(1 + idx, mask | bit, change); }
            else { res = 1 + Dfs(1 + idx, bit, change); }
            if (!change)
            {
                for (int i = 0; i < 26; i++)
                {
                    bit = 1 << i;
                    int val;
                    if (int.PopCount(mask | bit) <= k) { val = Dfs(1 + idx, mask | bit, true); }
                    else { val = 1 + Dfs(1 + idx, bit, true); }
                    res = int.Max(res, val);
                }
            }
            memo.Add(key, res);
            return res;
        }
    }
}


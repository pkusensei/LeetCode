using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CanIWin(int maxChoosableInteger, int desiredTotal)
    {
        if (desiredTotal <= 0) { return true; }
        if ((1 + maxChoosableInteger) * maxChoosableInteger / 2 < desiredTotal) { return false; }
        Dictionary<int, bool> memo = [];
        return Dfs(0, desiredTotal);

        bool Dfs(int mask, int total)
        {
            if (total <= 0) { return false; }
            if (memo.TryGetValue(mask, out bool v)) { return v; }
            for (int bit = 0; bit < maxChoosableInteger; bit++)
            {
                if (((mask >> bit) & 1) == 0)
                {
                    int nmask = mask | (1 << bit);
                    int ntotal = total - 1 - bit;
                    bool res = ntotal <= 0 || !Dfs(nmask, ntotal);
                    if (res)
                    {
                        memo.Add(mask, res);
                        return true;
                    }
                }
            }
            memo[mask] = false;
            return false;
        }
    }
}
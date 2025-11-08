using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumOneBitOperations(int n)
    {
        // 100 101 111 110 010 011 001 000
        // 4               2       1
        Dictionary<int, int> memo = Enumerable.Range(0, 30)
            .Select(p => ((int)Math.Pow(2, p), (int)Math.Pow(2, 1 + p) - 1))
            .ToDictionary(kv => kv.Item1, kv => kv.Item2);
        return Dfs(n);

        int Dfs(int x)
        {
            if (x <= 1) { return x; }
            if (memo.TryGetValue(x, out var v)) { return v; }
            int p = int.Log2(x);
            int res = Dfs((int)Math.Pow(2, p)) - Dfs(x - (int)Math.Pow(2, p));
            memo.TryAdd(x, res);
            return res;
        }
    }
}


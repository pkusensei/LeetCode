using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumTrees(int n)
    {
        Span<int> memo = stackalloc int[1 + n];
        memo[..2].Fill(1);
        return Backtrack(n, memo);

        static int Backtrack(int n, Span<int> memo)
        {
            if (memo[n] > 0) { return memo[n]; }
            int res = 0;
            for (int i = 0; i < n; i++)
            {
                var a = Backtrack(i, memo);
                var b = Backtrack(n - 1 - i, memo);
                res += a * b;
            }
            memo[n] = res;
            return res;
        }
    }
}
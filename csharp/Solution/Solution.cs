using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumDecodings(string s)
    {
        Span<int> memo = stackalloc int[1 + s.Length];
        memo.Fill(-1);
        return Dfs(s, memo);

        static int Dfs(ReadOnlySpan<char> s, Span<int> memo)
        {
            if (s.IsEmpty) { return 1; }
            if (memo[s.Length] > -1) { return memo[s.Length]; }
            int res = 0;
            if (s[0] == '0') { res = 0; }
            else if (s[0] == '1')
            {
                res = Dfs(s[1..], memo);
                if (s.Length >= 2) { res += Dfs(s[2..], memo); }
            }
            else if (s[0] == '2')
            {
                res = Dfs(s[1..], memo);
                if (s.Length >= 2 && s[1] <= '6') { res += Dfs(s[2..], memo); }
            }
            else { res = Dfs(s[1..], memo); }
            memo[s.Length] = res;
            return res;
        }
    }
}
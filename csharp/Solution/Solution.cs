using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] PathsWithMaxScore(IList<string> board)
    {
        const int M = 1_000_000_007;
        ReadOnlySpan<(int, int)> D = [(-1, 0), (0, -1), (-1, -1)];

        int n = board.Count;
        (int score, int count)[,] dp = new (int, int)[n, n];
        dp[0, 0] = (0, 1);
        for (int r = 0; r < n; r++)
        {
            for (int c = 0; c < n; c++)
            {
                if (board[r][c] == 'E' || board[r][c] == 'X') { continue; }
                foreach (var (dr, dc) in D)
                {
                    int pr = r + dr;
                    int pc = c + dc;
                    if (pr >= 0 && pc >= 0)
                    {
                        int prev = dp[pr, pc].score;
                        if (dp[r, c].score < prev) { dp[r, c] = (prev, 0); }
                        if (dp[r, c].score == prev)
                        {
                            dp[r, c].count = (dp[r, c].count + dp[pr, pc].count) % M;
                        }
                    }
                }
                if (char.IsAsciiDigit(board[r][c]))
                {
                    dp[r, c].score += board[r][c] - '0';
                }
            }
        }
        (int max, int count) = dp[n - 1, n - 1];
        return count > 0 ? [max, count] : [0, 0];
    }
}

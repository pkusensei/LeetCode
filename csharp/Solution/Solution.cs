using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double KnightProbability(int n, int k, int row, int column)
    {
        double[,,] memo = new double[1 + k, n, n];
        for (int i1 = 0; i1 <= k; i1++)
        {
            for (int r = 0; r < n; r++)
            {
                for (int c = 0; c < n; c++)
                {
                    memo[i1, r, c] = -1.0;
                }
            }
        }
        return Dfs(k, row, column, 1.0);

        double Dfs(int k, int row, int col, double f)
        {
            if (row < 0 || n <= row || col < 0 || n <= col) { return 0.0; }
            if (k == 0) { return f; }
            if (memo[k, row, col] > -1.0) { return memo[k, row, col]; }
            Span<(int, int)> deltas = [(-2, -1), (-1, -2), (2, 1), (1, 2),
                                        (2, -1), (-1,2), (-2, 1), (1, -2)];
            double res = 0.0;
            foreach (var (dr, dc) in deltas)
            {
                res += Dfs(k - 1, row + dr, col + dc, f / 8.0);
            }
            memo[k, row, col] = res;
            return res;
        }
    }
}

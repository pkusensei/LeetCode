using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    const int M = 1_000_000_007;
    int Rows { get; set; }
    int Cols { get; set; }
    int[,] Memo { get; set; }

    public int ColorTheGrid(int m, int n)
    {
        Rows = m;
        Cols = n;
        Memo = new int[Cols, 1 << (2 * Rows)];
        for (int i = 0; i < Cols; i++)
        {
            for (int j = 0; j < 1 << (2 * Rows); j++)
            {
                Memo[i, j] = -1;
            }
        }
        return Dfs(0, 0, 0, 0);
    }

    int Dfs(int row_idx, int col_idx, int left_mask, int mask)
    {
        if (col_idx == Cols) { return 1; }
        if (row_idx == Rows) { return Dfs(0, 1 + col_idx, mask, 0); }
        if (row_idx == 0 && Memo[col_idx, left_mask] > -1) { return Memo[col_idx, left_mask]; }
        int up = row_idx == 0 ? 0 : (mask & 0b11);
        int left = (left_mask >> (2 * (Rows - row_idx - 1))) & 0b11;
        int res = 0;
        for (int color = 1; color < 4; color++)
        {
            if (color != up && color != left)
            {
                int new_mask = (mask << 2) | color;
                res = (res + Dfs(1 + row_idx, col_idx, left_mask, new_mask)) % M;
            }
        }
        if (row_idx == 0) { Memo[col_idx, left_mask] = res; }
        return res;
    }
}

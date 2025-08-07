using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxCollectedFruits(int[][] fruits)
    {
        int n = fruits.Length;
        int[,] memo = new int[n, n];
        int res = 0;
        for (int i = 0; i < n; i++)
        {
            res += fruits[i][i];
            for (int j = 0; j < n; j++)
            {
                memo[i, j] = -1;
            }
        }
        return res + TopRight(0, n - 1) + BottomLeft(n - 1, 0);

        int TopRight(int row, int col)
        {
            if (row < 0 || n <= row || col < 0 || n <= col) { return 0; }
            if (row == n - 1 && col == n - 1) { return 0; }
            if (row >= col) { return 0; }
            if (memo[row, col] > -1) { return memo[row, col]; }
            int curr = 0;
            foreach (var nc in new[] { col - 1, col, 1 + col })
            {
                curr = int.Max(curr, TopRight(1 + row, nc));
            }
            curr += fruits[row][col];
            memo[row, col] = curr;
            return curr;
        }

        int BottomLeft(int row, int col)
        {
            if (row < 0 || n <= row || col < 0 || n <= col) { return 0; }
            if (row == n - 1 && col == n - 1) { return 0; }
            if (col >= row) { return 0; }
            if (memo[row, col] > -1) { return memo[row, col]; }
            int curr = 0;
            foreach (var nr in new[] { row - 1, row, 1 + row })
            {
                curr = int.Max(curr, BottomLeft(nr, 1 + col));
            }
            curr += fruits[row][col];
            memo[row, col] = curr;
            return curr;
        }
    }
}

using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    static (int, int)[] deltas = [.. Enumerable.Range(-1, 3)
                                            .SelectMany(x => Enumerable.Range(-1, 3).Select(y => (x, y)))
                                            .Where(p=>p.x!=0||p.y!=0)];

    public void GameOfLife(int[][] board)
    {
        int rows = board.Length;
        int cols = board[0].Length;
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                int around = deltas.Select(p =>
                {
                    int nr = p.Item1 + r;
                    int nc = p.Item2 + c;
                    if (0 <= nr && nr < rows && 0 <= nc && nc < cols) { return board[nr][nc] & 1; }
                    return 0;
                }).Sum();
                if (board[r][c] == 1 && (around == 2 || around == 3)) { board[r][c] += 2; }
                else if (board[r][c] == 0 && around == 3) { board[r][c] += 2; }
            }
        }
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                board[r][c] /= 2;
            }
        }
    }
}
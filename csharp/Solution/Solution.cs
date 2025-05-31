using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SnakesAndLadders(int[][] board)
    {
        int n = board.Length;
        Array.Reverse(board);
        Queue<(int, int, int)> queue = [];
        queue.Enqueue((0, 0, 0));
        bool[,] seen = new bool[n, n];
        seen[0, 0] = true;
        while (queue.TryDequeue(out var item))
        {
            (var row, var col, var dist) = item;
            if (n * n == CoordToLabel(row, col)) { return dist; }
            foreach (var (nr, nc) in Next(row, col))
            {
                if (!seen[nr, nc])
                {
                    seen[nr, nc] = true;
                    queue.Enqueue((nr, nc, 1 + dist));
                }
            }
        }
        return -1;

        int CoordToLabel(int row, int col)
        {
            if ((row & 1) == 0) { return n * row + col + 1; }
            return n * (1 + row) - col;
        }

        (int, int) LabelToCoord(int label)
        {
            int row = (label - 1) / n;
            int col = (row & 1) == 0 ? (label - 1) % n : n - 1 - (label - 1) % n;
            return (row, col);
        }

        IEnumerable<(int, int)> Next(int row, int col)
        {
            int label = CoordToLabel(row, col);
            for (int node = 1 + label; node <= Math.Min(label + 6, n * n); node += 1)
            {
                (int nr, int nc) = LabelToCoord(node);
                if (board[nr][nc] == -1) { yield return (nr, nc); }
                else { yield return LabelToCoord(board[nr][nc]); }
            }
        }
    }
}

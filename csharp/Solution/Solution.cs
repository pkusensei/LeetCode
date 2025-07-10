using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<string>> SolveNQueens(int n)
    {
        List<IList<string>> res = [];
        if (n == 1) { return [["Q"]]; }
        if (n == 2 || n == 3) { return res; }
        List<List<int>> board = [];
        Backtrack(0, []);
        foreach (var item in board)
        {
            List<string> curr = [];
            foreach (var col in item)
            {
                char[] chs = [.. Enumerable.Repeat('.', n)];
                chs[col] = 'Q';
                curr.Add(new(chs));
            }
            res.Add(curr);
        }
        return res;

        void Backtrack(int row, List<int> cols)
        {
            if (row == n)
            {
                board.Add([.. cols]);
                return;
            }
            for (int col = 0; col < n; col++)
            {
                if (Check(cols, col))
                {
                    cols.Add(col);
                    Backtrack(1 + row, cols);
                    cols.RemoveAt(cols.Count - 1);
                }
            }
        }

        static bool Check(List<int> cols, int col)
        {
            for (int row = 0; row < cols.Count; row += 1)
            {
                if (col == cols[row]
                    || Math.Abs(row - cols.Count) == Math.Abs(col - cols[row]))
                {
                    return false;
                }
            }
            return true;
        }
    }
}
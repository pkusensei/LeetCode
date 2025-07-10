using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int TotalNQueens(int n)
    {
        if (n == 1) { return 1; }
        if (n == 2 || n == 3) { return 0; }
        int res = 0;
        Backtrack(0, []);
        return res;

        void Backtrack(int row, List<int> cols)
        {
            if (row == n)
            {
                res += 1;
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
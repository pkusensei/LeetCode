using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] FindDiagonalOrder(int[][] mat)
    {
        int rows = mat.Length;
        int cols = mat[0].Length;
        Dictionary<int, List<int>> dict = [];
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (!dict.TryAdd(r + c, [mat[r][c]])) { dict[r + c].Add(mat[r][c]); }
            }
        }
        List<int> res = new(rows * cols);
        foreach (var (k, v) in dict)
        {
            if ((k & 1) == 0) { v.Reverse(); }
            res.AddRange(v);
        }
        return [.. res];
    }
}
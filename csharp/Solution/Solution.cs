using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumSubmat(int[][] mat)
    {
        int rows = mat.Length;
        int cols = mat[0].Length;
        int[,] dp = new int[rows, cols];
        int res = 0;
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (mat[r][c] == 0) { continue; }
                dp[r, c] = 1;
                // accumulate flat lines
                if (c > 0) { dp[r, c] += dp[r, c - 1]; }
                int curr = dp[r, c];
                for (int i = r; i >= 0; i -= 1)
                {
                    // This `Min` call does the magic
                    // e.g for matrix
                    // 0 1 1  <== accumulates to 2
                    // 1 1 1  <== accumulates to 3
                    // The min determines valid rectangles with height to upper row i
                    curr = int.Min(curr, dp[i, c]);
                    res += curr; // after curr==0 it has no effect
                }
            }
        }
        return res;
    }

    public int WithMonoStack(int[][] mat)
    {
        int cols = mat[0].Length;
        int[] heights = new int[cols];
        int res = 0;
        foreach (var row in mat)
        {
            for (int c = 0; c < cols; c++)
            {
                heights[c] = row[c] == 0 ? 0 : 1 + heights[c];
            }
            Stack<(int idx, int area, int height)> st = [];
            st.Push((-1, 0, -1));
            for (int c = 0; c < cols; c++)
            {
                while (st.TryPeek(out var top) && top.height >= heights[c])
                {
                    st.Pop();
                }
                var prev = st.Peek();
                // A rectangle with prev.height
                int area = prev.area + (c - prev.idx) * heights[c];
                res += area;
                st.Push((c, area, heights[c]));
            }
        }
        return res;
    }
}
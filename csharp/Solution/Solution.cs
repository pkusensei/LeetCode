using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxSumSubmatrix(int[][] matrix, int k)
    {
        int rows = matrix.Length;
        int cols = matrix[0].Length;
        int[,] prefix = new int[1 + rows, 1 + cols];
        for (int r = 0; r < rows; r++)
        {
            int curr = 0;
            for (int c = 0; c < cols; c++)
            {
                curr += matrix[r][c];
                prefix[1 + r, 1 + c] = curr + prefix[r, 1 + c];
            }
        }
        int res = int.MinValue;
        for (int r1 = 0; r1 < rows; r1++)
        {
            for (int c1 = 0; c1 < cols; c1++)
            {
                for (int r2 = r1; r2 < rows; r2++)
                {
                    for (int c2 = c1; c2 < cols; c2++)
                    {
                        int curr = prefix[1 + r2, 1 + c2] + prefix[r1, c1]
                                   - prefix[1 + r2, c1] - prefix[r1, 1 + c2];
                        if (curr <= k) { res = int.Max(res, curr); }
                    }
                }
            }
        }
        return res;
    }

    public int WithKadanes(int[][] mat, int k)
    {
        int rows = mat.Length;
        int cols = mat[0].Length;
        int res = int.MinValue;
        for (int top = 0; top < rows; top++)
        {
            int[] row_sum = new int[cols];
            for (int bot = top; bot < rows; bot++)
            {
                int kadane = 0;
                int max_kadane = int.MinValue;
                for (int i = 0; i < cols; i++)
                {
                    row_sum[i] += mat[bot][i];
                    kadane = int.Max(row_sum[i], kadane + row_sum[i]);
                    max_kadane = int.Max(max_kadane, kadane);
                }
                if (max_kadane <= k)
                {
                    res = int.Max(res, max_kadane);
                    continue;
                }
                SortedSet<int> set = [0];
                int running_sum = 0;
                foreach (var num in row_sum)
                {
                    running_sum += num;
                    var temp = set.GetViewBetween(running_sum - k, int.MaxValue);
                    if (temp.Count > 0)
                    {
                        res = int.Max(res, running_sum - temp.Min);
                    }
                    set.Add(running_sum);
                }
            }
        }
        return res;

    }
}
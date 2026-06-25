using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumSubmatrixSumTarget(int[][] matrix, int target)
    {
        int rows = matrix.Length;
        int cols = matrix[0].Length;
        int[,] prefix = new int[rows, cols];
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                prefix[r, c] = matrix[r][c];
                if (c > 0) { prefix[r, c] += prefix[r, c - 1]; }
            }
        }
        int res = 0;
        for (int left = 0; left < cols; left++)
        {
            for (int right = left; right < cols; right++)
            {
                Dictionary<int, int> dict = new() { [0] = 1 };
                int curr = 0;
                for (int r = 0; r < rows; r++)
                {
                    curr += prefix[r, right];
                    if (left > 0) { curr -= prefix[r, left - 1]; }
                    if (dict.TryGetValue(curr - target, out var v)) { res += v; }
                    if (!dict.TryAdd(curr, 1)) { dict[curr] += 1; }
                }
            }
        }
        return res;
    }
}

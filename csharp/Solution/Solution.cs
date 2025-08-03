using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int KthSmallest(int[][] matrix, int k)
    {
        int n = matrix.Length;
        PriorityQueue<(int, int), int> pq = new();
        for (int i = 0; i < n; i++)
        {
            pq.Enqueue((0, i), matrix[0][i]);
        }
        while (k > 0 && pq.TryDequeue(out var item, out _))
        {
            (int r, int c) = item;
            k -= 1;
            if (k == 0) { return matrix[r][c]; }
            if (1 + r < n) { pq.Enqueue((1 + r, c), matrix[1 + r][c]); }
        }
        return -1;
    }

    public int WithBinarySearch(int[][] mat, int k)
    {
        int left = mat[0][0];
        int right = mat[^1][^1];
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (Count(mid) < k) { left = 1 + mid; }
            else { right = mid; }
        }
        return left;

        int Count(int mid)
        {
            int n = mat.Length;
            int r = n - 1;
            int c = 0;
            int count = 0;
            while (r >= 0 && c < n)
            {
                int curr = mat[r][c];
                if (curr <= mid)
                {
                    count += 1 + r;
                    c += 1;
                }
                else
                {
                    r -= 1;
                }
            }
            return count;
        }
    }
}
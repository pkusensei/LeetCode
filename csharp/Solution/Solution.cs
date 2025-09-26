using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindKthNumber(int m, int n, int k)
    {
        int left = 1;
        int right = m * n;
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (Count(mid) >= k) { right = mid; }
            else { left = 1 + mid; }
        }
        return left;

        int Count(int mid)
        {
            int acc = 0;
            for (int row = 1; row <= m; row++)
            {
                // mid/row is
                // on this `row`, how many numbers are <= mid
                acc += int.Min(mid / row, n);
            }
            return acc;
        }
    }
}
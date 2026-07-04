using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinHeightShelves(int[][] books, int shelfWidth)
    {
        int n = books.Length;
        int[] dp = new int[1 + n];
        Array.Fill(dp, int.MaxValue >> 1, 1, n);
        for (int right = 0; right < n; right++)
        {
            int thick = books[right][0];
            int height = books[right][1];
            dp[1 + right] = int.Min(dp[1 + right], dp[right] + height);

            int curr_thick = thick;
            for (int left = right - 1; left >= 0; left -= 1)
            {
                curr_thick += books[left][0];
                if (curr_thick > shelfWidth) { break; }
                height = int.Max(height, books[left][1]);
                dp[1 + right] = int.Min(dp[1 + right], height + dp[left]);
            }
        }
        return dp[n];
    }
}

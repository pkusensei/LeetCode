using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SuperEggDrop(int k, int n)
    {
        int[] prev = new int[1 + k];
        int moves = 0;
        for (; prev[k] < n; moves += 1)
        {
            int[] curr = new int[1 + k];
            for (int kk = 1; kk <= k; kk++)
            {
                curr[kk] = 1 + prev[kk - 1] + prev[kk];
            }
            prev = curr;
        }
        return moves;
    }
}
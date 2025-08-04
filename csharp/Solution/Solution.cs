using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LastRemaining(int n)
    {
        return Dfs(n, true);

        static int Dfs(int n, bool forward)
        {
            if (n == 1) { return 1; }
            // Remove elements on odd indices
            if (forward) { return 2 * Dfs(n / 2, !forward); }
            else
            {
                // Remove elements on odd indices
                if ((n & 1) == 1) { return 2 * Dfs(n / 2, !forward); }
                // [1,2,3,4,5,6] remove even indices to get [1,3,5]
                // [1,3,5] == [2*1-1, 2*2-1, 2*3-1]
                else { return 2 * Dfs(n / 2, !forward) - 1; }
            }
        }
    }
}
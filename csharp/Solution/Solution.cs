using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinSteps(int n)
    {
        if (n == 1) { return 0; }
        int[,] memo = new int[1 + n, 1 + n / 2];
        return 1 + Dfs(1, 1);

        int Dfs(int curr, int clipboard)
        {
            if (curr == n) { return 0; }
            if (curr > n) { return int.MaxValue / 2; }
            if (memo[curr, clipboard] > 0) { return memo[curr, clipboard]; }
            int paste = 1 + Dfs(curr + clipboard, clipboard);
            int copy_paste = 2 + Dfs(2 * curr, curr);
            memo[curr, clipboard] = int.Min(paste, copy_paste);
            return memo[curr, clipboard];
        }
    }
}
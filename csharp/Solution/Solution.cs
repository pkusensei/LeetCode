using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int RemoveBoxes(int[] boxes)
    {
        int n = boxes.Length;
        int[,,] memo = new int[n, n, n];
        for (int a = 0; a < n; a++)
        {
            for (int b = 0; b < n; b++)
            {
                for (int c = 0; c < n; c++)
                {
                    memo[a, b, c] = -1;
                }
            }
        }
        return Dfs(0, n - 1, 0);

        int Dfs(int left, int right, int streak)
        {
            if (left > right) { return 0; }
            if (memo[left, right, streak] > -1) { return memo[left, right, streak]; }
            int curr_streak = streak;
            int left_end = left;
            while (left_end <= right && boxes[left_end] == boxes[left])
            {
                left_end += 1;
                curr_streak += 1; // Greedily accumulate streak
            }
            // Option 1) Cut at end of streak
            int res = curr_streak * curr_streak + Dfs(left_end, right, 0);
            for (int i = left_end; i <= right; i++)
            {
                if (boxes[i] == boxes[left])
                {
                    // Option 2) Continue this streak with window to the right
                    // ..1,1,2,2,1,1.. is seen as 
                    // 2,2 and ..1,1,1,1..
                    int curr = Dfs(left_end, i - 1, 0) + Dfs(i, right, curr_streak);
                    res = int.Max(res, curr);
                }
            }
            memo[left, right, streak] = res;
            return res;
        }
    }
}

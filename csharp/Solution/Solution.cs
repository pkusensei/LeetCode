using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] NumsSameConsecDiff(int n, int k)
    {
        Queue<(int val, int len)> queue = [];
        List<int> res = [];
        for (int i = 1; i < 10; i++)
        {
            queue.Enqueue((i, 1));
        }
        while (queue.TryDequeue(out var item))
        {
            if (item.len == n)
            {
                res.Add(item.val);
                continue;
            }
            int prev = item.val % 10;
            for (int d = 0; d < 10; d++)
            {
                if (int.Abs(prev - d) == k)
                {
                    queue.Enqueue((10 * item.val + d, 1 + item.len));
                }
            }
        }
        return [.. res];
    }
}

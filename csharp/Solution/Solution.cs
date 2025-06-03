using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxCandies(int[] status, int[] candies, int[][] keys, int[][] containedBoxes, int[] initialBoxes)
    {
        int n = status.Length;
        Span<bool> available = stackalloc bool[n];
        Queue<int> queue = [];
        foreach (var item in initialBoxes)
        {
            if (status[item] > 0) { queue.Enqueue(item); }
            else { available[item] = true; }
        }
        int res = 0;
        while (queue.TryDequeue(out var node))
        {
            res += candies[node];
            foreach (var item in keys[node])
            {
                status[item] = 1;
                if (status[item] > 0 && available[item])
                {
                    queue.Enqueue(item);
                    available[item] = false;
                }
            }
            foreach (var item in containedBoxes[node])
            {
                available[item] = true;
                if (status[item] > 0 && available[item])
                {
                    queue.Enqueue(item);
                    available[item] = false;
                }
            }
        }
        return res;
    }
}

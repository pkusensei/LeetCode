using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] GardenNoAdj(int n, int[][] paths)
    {
        List<List<int>> adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        foreach (var item in paths)
        {
            adj[item[0] - 1].Add(item[1] - 1);
            adj[item[1] - 1].Add(item[0] - 1);
        }
        int[] res = new int[n];
        for (int i = 0; i < n; i++)
        {
            if (res[i] > 0) { continue; }
            Queue<int> queue = [];
            queue.Enqueue(i);
            while (queue.TryDequeue(out var node))
            {
                for (int c = 1; c <= 4; c++)
                {
                    if (adj[node].All(next => res[next] != c))
                    {
                        res[node] = c;
                        break;
                    }
                }
                foreach (var next in adj[node])
                {
                    if (res[next] == 0) { queue.Enqueue(next); }
                }
            }
        }
        return [.. res];
    }
}

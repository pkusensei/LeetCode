using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] ShortestAlternatingPaths(int n, int[][] redEdges, int[][] blueEdges)
    {
        var red_adj = Build(n, redEdges);
        var blue_adj = Build(n, blueEdges);
        int[,] dist = new int[2, n];
        for (int i = 1; i < n; i++)
        {
            dist[0, i] = int.MaxValue >> 1;
            dist[1, i] = int.MaxValue >> 1;
        }
        Queue<(int node, int color, int step)> queue = [];
        for (int i = 0; i < 2; i++)
        {
            queue.Enqueue((0, i, 0));
        }
        while (queue.TryDequeue(out var item))
        {
            if (item.step > dist[item.color, item.node]) { continue; }
            var nadj = item.color == 0 ? blue_adj : red_adj;
            foreach (var next in nadj[item.node])
            {
                int nstep = 1 + item.step;
                if (nstep < dist[1 - item.color, next])
                {
                    dist[1 - item.color, next] = nstep;
                    queue.Enqueue((next, 1 - item.color, nstep));
                }
            }
        }
        int[] res = new int[n];
        for (int i = 0; i < n; i++)
        {
            int v = int.Min(dist[0, i], dist[1, i]);
            res[i] = v >= (int.MaxValue >> 1) ? -1 : v;
        }
        return res;

        static List<int>[] Build(int n, int[][] edges)
        {
            List<int>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
            foreach (var e in edges)
            {
                adj[e[0]].Add(e[1]);
            }
            return adj;
        }
    }
}

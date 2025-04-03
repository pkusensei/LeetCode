using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    const int INF = 2_000_000_000;
    public int[][] ModifiedGraphEdges(int n, int[][] edges, int source, int destination, int target)
    {
        long curr = Dijkstra();
        if (curr < target) { return []; }
        bool matched = curr == target;
        foreach (var edge in edges)
        {
            if (edge[2] > 0) { continue; }
            edge[2] = matched ? INF : 1;
            if (!matched)
            {
                curr = Dijkstra();
                if (curr <= target)
                {
                    matched = true;
                    edge[2] += target - (int)curr;
                }
            }
        }
        return matched ? edges : [];

        long Dijkstra()
        {
            int[,] adj = new int[n, n];
            foreach (var edge in edges)
            {
                if (edge[2] > 0)
                {
                    adj[edge[0], edge[1]] = edge[2];
                    adj[edge[1], edge[0]] = edge[2];
                }
            }
            long[] dists = new long[n];
            Array.Fill(dists, INF);
            dists[source] = 0;
            var pq = new PriorityQueue<int, long>();
            pq.Enqueue(source, 0);
            while (pq.TryDequeue(out int node, out long dist))
            {
                if (node == destination) { return dist; }
                if (dist > dists[node]) { continue; }
                for (int next = 0; next < n; next += 1)
                {
                    if (adj[node, next] > 0)
                    {
                        long nc = dist + adj[node, next];
                        if (nc < dists[next])
                        {
                            dists[next] = nc;
                            pq.Enqueue(next, nc);
                        }
                    }
                }
            }
            return dists[destination];
        }
    }
}

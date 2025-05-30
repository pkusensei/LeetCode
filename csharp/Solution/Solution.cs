using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ClosestMeetingNode(int[] edges, int node1, int node2)
    {
        var arr1 = Bfs(node1);
        var arr2 = Bfs(node2);
        int min_dist = int.MaxValue;
        int res = -1;
        for (int i = 0; i < edges.Length; i++)
        {
            if (arr1[i] > -1 && arr2[i] > -1)
            {
                int d = Math.Max(arr1[i], arr2[i]);
                if (d < min_dist)
                {
                    min_dist = d;
                    res = i;
                }
            }
        }
        return res;

        int[] Bfs(int start)
        {
            int n = edges.Length;
            int[] res = new int[n];
            Array.Fill(res, -1);
            res[start] = 0;
            Queue<(int, int)> queue = [];
            queue.Enqueue((start, 0));
            while (queue.TryDequeue(out var item))
            {
                (var node, var dist) = item;
                int next = edges[node];
                if (next > -1 && res[next] == -1)
                {
                    res[next] = 1 + dist;
                    queue.Enqueue((next, 1 + dist));
                }
            }
            return res;
        }
    }
}

using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxKDivisibleComponents(int n, int[][] edges, int[] values, int k)
    {
        List<List<int>> adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        foreach (var edge in edges)
        {
            adj[edge[0]].Add(edge[1]);
            adj[edge[1]].Add(edge[0]);
        }
        int res = 1;
        Dfs(0, -1);
        return res;

        long Dfs(int node, int prev)
        {
            long sum = values[node];
            foreach (var next in adj[node])
            {
                if (next != prev)
                {
                    var s = Dfs(next, node);
                    if (s % k == 0) { res += 1; }
                    else { sum += s; }
                }
            }
            return sum;
        }
    }
}

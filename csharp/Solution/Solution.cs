using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Net.Sockets;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> CriticalConnections(int n, IList<IList<int>> connections)
    {
        List<int>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        foreach (var item in connections)
        {
            adj[item[0]].Add(item[1]);
            adj[item[1]].Add(item[0]);
        }
        int[] tin = new int[n];
        int[] min_t = new int[n];
        int time = 0;
        List<IList<int>> res = [];
        Dfs(0, n);
        return res;

        void Dfs(int node, int prev)
        {
            time += 1;
            tin[node] = time;
            min_t[node] = time;
            foreach (var next in adj[node])
            {
                if (next == prev) { continue; }
                if (tin[next] > 0) { min_t[node] = int.Min(min_t[node], tin[next]); }
                else
                {
                    Dfs(next, node);
                    min_t[node] = int.Min(min_t[node], min_t[next]);
                    if (min_t[next] > tin[node]) { res.Add([node, next]); }
                }
            }
        }
    }
}
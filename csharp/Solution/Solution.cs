using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumScore(int[] nums, int[][] edges)
    {
        int n = nums.Length;
        List<int>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        foreach (var e in edges)
        {
            adj[e[0]].Add(e[1]);
            adj[e[1]].Add(e[0]);
        }
        int[] xors = new int[n];
        int[] last = new int[n];
        int node_id = 0;
        int total_xor = Dfs(0, -1);
        int res = int.MaxValue;
        for (int i1 = 1; i1 < n; i1++)
        {
            for (int i2 = 1 + i1; i2 < n; i2++)
            {
                int v1 = i2 < last[i1] ? total_xor ^ xors[i1] : total_xor ^ xors[i1] ^ xors[i2];
                int v2 = i2 < last[i1] ? xors[i1] ^ xors[i2] : xors[i1];
                int min = int.Min(xors[i2], int.Min(v1, v2));
                int max = int.Max(xors[i2], int.Max(v1, v2));
                res = int.Min(res, max - min);
            }
        }
        return res;

        int Dfs(int node, int prev)
        {
            int curr_val = nums[node];
            int curr_id = node_id;
            node_id += 1;
            foreach (var next in adj[node])
            {
                if (next != prev) { curr_val ^= Dfs(next, node); }
            }
            last[curr_id] = node_id;
            xors[curr_id] = curr_val;
            return curr_val;
        }
    }
}
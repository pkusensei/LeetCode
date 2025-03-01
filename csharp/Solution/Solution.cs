using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaximumInvitations(int[] favorite)
    {
        int n = favorite.Length;
        var indegs = new int[n];
        foreach (var fav in favorite) { indegs[fav] += 1; }
        Queue<int> queue = [];
        foreach (var (i, deg) in indegs.Select((v, i) => (i, v)))
        {
            if (deg == 0) { queue.Enqueue(i); }
        }
        var depth = new int[n];
        Array.Fill(depth, 1);
        // Topo sort to remove non-cycle nodes
        // And find their depths
        while (queue.TryDequeue(out var curr))
        {
            int next = favorite[curr];
            depth[next] = Math.Max(depth[next], 1 + depth[curr]);
            indegs[next] -= 1;
            if (indegs[next] == 0) { queue.Enqueue(next); }
        }
        int long_cycle = 0;
        int two_cycles = 0;
        for (int i = 0; i < n; i++)
        {
            if (indegs[i] == 0) { continue; }
            int cycle_len = 0;
            int curr = i;
            while (indegs[curr] != 0)
            {
                indegs[curr] = 0;
                cycle_len += 1;
                curr = favorite[curr];
            }
            if (cycle_len == 2) { two_cycles += depth[i] + depth[favorite[i]]; }
            else { long_cycle = Math.Max(long_cycle, cycle_len); }
        }
        return Math.Max(long_cycle, two_cycles);
    }
}

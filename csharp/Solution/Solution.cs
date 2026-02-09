using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinRefuelStops(int target, int startFuel, int[][] stations)
    {
        int pos = startFuel;
        // max queue
        PriorityQueue<int, int> pq = new(Comparer<int>.Create((a, b) => b.CompareTo(a)));
        int res = 0;
        foreach (var s in stations)
        {
            while (pos < s[0] && pq.TryDequeue(out int f, out _))
            {
                pos += f;
                res += 1;
            }
            if (pos >= s[0]) { pq.Enqueue(s[1], s[1]); }
        }
        while (pos < target && pq.TryDequeue(out int f, out _))
        {
            pos += f;
            res += 1;
        }
        return pos < target ? -1 : res;
    }
}
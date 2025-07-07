using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxEvents(int[][] events)
    {
        Array.Sort(events, (a, b) =>
        {
            if (a[0] == b[0]) { return a[1].CompareTo(b[1]); }
            return a[0].CompareTo(b[0]);
        });
        int max = events.Select(e => e[1]).Max();
        PriorityQueue<int, int> pq = new();
        int res = 0;
        int idx = 0;
        // For each day
        for (int day = 0; day <= max; day++)
        {
            while (pq.TryPeek(out int top, out _) && top < day)
            {
                pq.Dequeue(); // Pop all expired candidates
            }
            // Push in potential events
            for (; idx < events.Length && events[idx][0] <= day; idx += 1)
            {
                pq.Enqueue(events[idx][1], events[idx][1]);
            }
            // Pick one to attend
            if (pq.TryDequeue(out _, out _)) { res += 1; }
        }
        return res;
    }
}

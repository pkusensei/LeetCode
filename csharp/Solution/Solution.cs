using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MostBooked(int n, int[][] meetings)
    {
        Array.Sort(meetings, (a, b) =>
        {
            if (a[0] == b[0]) { return a[1].CompareTo(b[1]); }
            return a[0].CompareTo(b[0]);
        });
        PriorityQueue<int, int> free = new();
        for (int i = 0; i < n; i++)
        {
            free.Enqueue(i, i);
        }
        PriorityQueue<int, (long end, int id)> inuse = new();
        int[] rooms = [.. Enumerable.Range(0, n).Select(_ => 0)];
        foreach (var meet in meetings)
        {
            int room;
            while (inuse.TryPeek(out room, out var prio) && prio.end <= meet[0])
            {
                inuse.Dequeue();
                free.Enqueue(room, room);
            }
            if (free.TryDequeue(out room, out _))
            {
                inuse.Enqueue(room, (meet[1], room));
            }
            else
            {
                inuse.TryDequeue(out room, out var prio);
                inuse.Enqueue(room, (prio.end + meet[1] - meet[0], room));
            }
            rooms[room] += 1;
        }
        int res = 0;
        for (int i = 0; i < n; i++)
        {
            if (rooms[i] > rooms[res]) { res = i; }
        }
        return res;
    }
}
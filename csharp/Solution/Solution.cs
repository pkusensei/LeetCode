using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LeastInterval(char[] tasks, int n)
    {
        // max heap, task - count
        PriorityQueue<char, int> ready = new(Comparer<int>.Create((a, b) => b.CompareTo(a)));
        foreach (var g in tasks.GroupBy(c => c))
        {
            ready.Enqueue(g.Key, g.Count());
        }
        // min heap, (task, count) - day
        PriorityQueue<(char ch, int count), int> wait = new();
        int res = 0;
        for (; ready.Count > 0 || wait.Count > 0; res++)
        {
            while (wait.TryPeek(out var item, out var prio) && prio + n < res)
            {
                wait.Dequeue();
                ready.Enqueue(item.ch, item.count);
            }
            if (ready.TryDequeue(out var ch, out var count))
            {
                if (count > 1) { wait.Enqueue((ch, count - 1), res); }
            }
        }
        return res;
    }
}
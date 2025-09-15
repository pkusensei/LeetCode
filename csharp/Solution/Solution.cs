using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ScheduleCourse(int[][] courses)
    {
        Array.Sort(courses, (a, b) => a[1].CompareTo(b[1]));
        int end = 0;
        PriorityQueue<int, int> pq = new();
        foreach (var c in courses)
        {
            if (end + c[0] <= c[1])
            {
                // this duration is applicable
                end += c[0];
                pq.Enqueue(c[0], -c[0]);
            }
            else if (pq.TryPeek(out int dur, out _) && dur > c[0])
            {
                // or is shorter ==> swap to get earlier end time
                pq.Dequeue();
                end -= dur;
                end += c[0];
                pq.Enqueue(c[0], -c[0]);
            }
        }
        return pq.Count;
    }
}
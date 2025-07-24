using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NthUglyNumber(int n)
    {
        PriorityQueue<int, int> pq = new();
        pq.Enqueue(1, 1);
        HashSet<int> seen = [];
        while (pq.TryDequeue(out var num, out _))
        {
            if (seen.Add(num))
            {
                if (seen.Count == n) { return num; }
                try
                {
                    checked
                    {
                        foreach (var p in new[] { 2, 3, 5 })
                        {
                            var curr = num * p;
                            pq.Enqueue(curr, curr);
                        }
                    }
                }
                catch (OverflowException) { }
            }
        }
        return -1;
    }
}
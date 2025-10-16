using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class KthLargest
{
    public KthLargest(int k, int[] nums)
    {
        PQ = new();
        K = k;
        foreach (var num in nums)
        {
            PQ.Enqueue(num, num);
            if (PQ.Count > K) { PQ.Dequeue(); }
        }
    }

    PriorityQueue<int, int> PQ { get; }
    int K { get; }

    public int Add(int val)
    {
        PQ.Enqueue(val, val);
        if (PQ.Count > K) { PQ.Dequeue(); }
        return PQ.Peek();
    }
}

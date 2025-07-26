using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class MedianFinder
{
    readonly PriorityQueue<int, int> small; // should be max heap
    readonly PriorityQueue<int, int> large;

    public MedianFinder()
    {
        small = new();
        large = new();
    }

    public void AddNum(int num)
    {
        if (!small.TryPeek(out var mid, out _) || mid > num)
        {
            small.Enqueue(num, -num);
        }
        else
        {
            large.Enqueue(num, num);
        }
        if (small.Count > 1 + large.Count)
        {
            int top = small.Dequeue();
            large.Enqueue(top, top);
        }
        if (small.Count < large.Count)
        {
            int top = large.Dequeue();
            small.Enqueue(top, -top);
        }
    }

    public double FindMedian()
    {
        if (small.Count == large.Count)
        {
            return (small.Peek() + large.Peek()) / 2.0;
        }
        else
        {
            return small.Peek();
        }
    }
}
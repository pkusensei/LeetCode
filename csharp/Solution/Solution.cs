using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double[] MedianSlidingWindow(int[] nums, int k)
    {
        // upper half
        PriorityQueue<int, int> min_heap = new();
        // lower half
        PriorityQueue<int, int> max_heap = new(Comparer<int>.Create((a, b) => b.CompareTo(a)));
        Dictionary<int, int> freq = [];
        int n = nums.Length;
        List<double> res = new(n - k);
        foreach (var num in nums[..k])
        {
            max_heap.Enqueue(num, num);
            min_heap.Enqueue(max_heap.Peek(), max_heap.Dequeue()); // Params eval-ed in order
            if (min_heap.Count > max_heap.Count)
            {
                max_heap.Enqueue(min_heap.Peek(), min_heap.Dequeue());
            }
        }
        double med = GetMedian();
        res.Add(med);
        for (int i = k; i < n; i++)
        {
            int prev = nums[i - k];
            // Lazy deletion
            if (!freq.TryAdd(prev, 1)) { freq[prev] += 1; }
            // Remove from lower half vs from upper half
            int balance = prev <= med ? -1 : 1;
            if (nums[i] <= med)
            {
                balance += 1; // add to lower half
                max_heap.Enqueue(nums[i], nums[i]);
            }
            else
            {
                balance -= 1;
                min_heap.Enqueue(nums[i], nums[i]);
            }
            if (balance < 0) // balance==-2 <== upper half has too many numbers
            {
                max_heap.Enqueue(min_heap.Peek(), min_heap.Dequeue());
            }
            else if (balance > 0) // balance==2
            {
                min_heap.Enqueue(max_heap.Peek(), max_heap.Dequeue());
            }
            while (max_heap.TryPeek(out int top, out _) && freq.TryGetValue(top, out var f) && f > 0)
            {
                freq[top] -= 1;
                max_heap.Dequeue();
            }
            while (min_heap.TryPeek(out int top, out _) && freq.TryGetValue(top, out var f) && f > 0)
            {
                freq[top] -= 1;
                min_heap.Dequeue();
            }
            med = GetMedian();
            res.Add(med);
        }
        return [.. res];

        double GetMedian()
        {
            if ((k & 1) == 1) { return max_heap.Peek(); }
            else { return ((double)max_heap.Peek() + min_heap.Peek()) / 2; }
        }
    }
}
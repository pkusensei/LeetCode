using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] SmallestRange(IList<IList<int>> nums)
    {
        int k = nums.Count;
        int min = int.MaxValue;
        int max = int.MinValue;
        PriorityQueue<(int val, int arr, int i), int> min_heap = new();
        int[] freq = new int[k];
        for (int arr = 0; arr < k; arr++)
        {
            min = int.Min(min, nums[arr][0]);
            max = int.Max(max, nums[arr][0]);
            freq[arr] = nums[arr].Count;
            int val = nums[arr][0];
            min_heap.Enqueue((val, arr, 0), val);
        }
        int right = max;
        while (min_heap.TryDequeue(out var item, out _))
        {
            (int left, int arr, int i) = item;
            if (right - left < max - min)
            {
                min = left;
                max = right;
            }
            if (1 + i < nums[arr].Count)
            {
                int val = nums[arr][1 + i];
                min_heap.Enqueue((val, arr, 1 + i), val);
                right = int.Max(right, val);
            }
            else { break; }
        }
        return [min, max];
    }
}
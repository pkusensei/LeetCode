using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] MaxSubsequence(int[] nums, int k)
    {
        PriorityQueue<(int i, int v), int> pq = new();
        for (int i = 0; i < nums.Length; i++)
        {
            pq.Enqueue((i, nums[i]), -nums[i]);
        }
        List<(int i, int v)> arr = [];
        while (arr.Count < k)
        {
            arr.Add(pq.Dequeue());
        }
        arr.Sort((a, b) => a.i.CompareTo(b.i));
        return [.. arr.Select(p => p.v)];
    }
}

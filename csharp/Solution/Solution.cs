using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] MaxSlidingWindow(int[] nums, int k)
    {
        int n = nums.Length;
        if (n <= k) { return [nums.Max()]; }
        LinkedList<int> deque = [];
        List<int> res = [];
        for (int i = 0; i < n; i++)
        {
            while (deque.Count > 0 && deque.Last.Value < nums[i])
            {
                deque.RemoveLast(); // mono decreasing deque
            }
            deque.AddLast(nums[i]);
            if (i >= k && deque.First.Value == nums[i - k]) { deque.RemoveFirst(); }
            if (i >= k - 1) { res.Add(deque.First.Value); }
        }
        return [.. res];
    }
}
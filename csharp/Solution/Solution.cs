using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    readonly Random rnd;
    readonly List<int> nums;

    public Solution(ListNode head)
    {
        rnd = new();
        nums = [];
        var curr = head;
        while (curr is not null)
        {
            nums.Add(curr.val);
            curr = curr.next;
        }
    }

    public int GetRandom()
    {
        var n = rnd.Next(0, nums.Count);
        return nums[n];
    }
}

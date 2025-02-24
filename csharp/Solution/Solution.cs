using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] NodesBetweenCriticalPoints(ListNode head)
    {
        var curr = head;
        List<int> nums = [];
        while (curr is not null)
        {
            nums.Add(curr.val);
            curr = curr.next;
        }
        List<int> points = [];
        for (int i = 1; i < nums.Count - 1; i++)
        {
            if ((nums[i - 1] < nums[i] && nums[i + 1] < nums[i]) || (nums[i - 1] > nums[i] && nums[i + 1] > nums[i]))
            {
                points.Add(i);
            }
        }
        if (points.Count < 2) { return [-1, -1]; }
        var min = int.MaxValue;
        for (int i = 0; i < points.Count - 1; i++)
        {
            min = Math.Min(min, points[i + 1] - points[i]);
        }
        return [min, points.Last() - points[0]];
    }
}


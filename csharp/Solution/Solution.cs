using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindDuplicate(int[] nums)
    {
        int slow = 0;
        int fast = 0;
        do
        {
            slow = nums[slow];
            fast = nums[nums[fast]];
        } while (slow != fast);
        slow = 0;
        do
        {
            slow = nums[slow];
            fast = nums[fast];
        } while (slow != fast);
        return slow;
    }

    public int WithFlip(int[] nums)
    {
        for (int i = 0; i < nums.Length; i++)
        {
            int next = int.Abs(nums[i]);
            if (nums[next] < 0) { return next; }
            nums[next] *= -1;
        }
        return -1;
    }
}
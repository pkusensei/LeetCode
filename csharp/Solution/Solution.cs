using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CircularArrayLoop(int[] nums)
    {
        int n = nums.Length;
        if (n < 2) { return false; }
        for (int i = 0; i < n; i++)
        {
            if (nums[i] == 0) { continue; }
            int slow = i;
            int fast = Step(i);
            // To ensure forward or backward direction, never both
            while (nums[i] * nums[fast] > 0 && nums[i] * nums[Step(fast)] > 0)
            {
                if (slow == fast)
                {
                    if (slow == Step(slow)) { break; } // single element loop
                    return true;
                }
                slow = Step(slow);
                fast = Step(Step(fast));
            }
            // This path is not a loop
            // Clear everything
            int curr = i;
            while (nums[curr] * nums[i] > 0)
            {
                int next = Step(curr);
                nums[curr] = 0;
                curr = next;
            }
        }
        return false;

        int Step(int curr) => ((curr + nums[curr]) % n + n) % n;
    }
}
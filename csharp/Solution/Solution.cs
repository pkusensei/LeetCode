using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool Find132pattern(int[] nums)
    {
        int n = nums.Length;
        if (n < 3) { return false; }
        Stack<int> st = []; // mono decreasing stack
        int curr_max = int.MinValue;
        foreach (int num in nums.Reverse())
        {
            // `curr_max` is always a value popped from stack
            // By the reverse nature of loop, 
            // `curr_max` is smaller than top of stack
            // which means [k] < [j] when j<k
            if (curr_max > num) { return true; } // found [i] < [k]
            while (st.TryPeek(out int top) && top < num)
            {
                st.Pop();
                curr_max = top;
            }
            st.Push(num);
        }
        return false;
    }
}
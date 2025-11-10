using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinOperations(int[] nums)
    {
        int res = 0;
        Stack<int> st = [];
        foreach (var num in nums)
        {
            if (num == 0)
            {
                st.Clear();
                continue;
            }
            while (st.TryPeek(out int top) && top > num)
            {
                st.Pop();
            }
            if (st.TryPeek(out int top_) && top_ == num) { continue; }
            st.Push(num);
            res += 1;
        }
        return res;
    }
}


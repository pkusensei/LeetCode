using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] NextLargerNodes(ListNode head)
    {
        Stack<(int i, int val)> st = [];
        List<int> res = [];
        int idx = 0;
        while (head is not null)
        {
            while (st.TryPeek(out var top) && top.val < head.val)
            {
                st.Pop();
                res[top.i] = head.val;
            }
            res.Add(0);
            st.Push((idx, head.val));
            idx += 1;
            head = head.next;
        }
        return [.. res];
    }
}


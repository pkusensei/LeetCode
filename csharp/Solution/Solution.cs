using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int PairSum(ListNode head)
    {
        var slow = head;
        var fast = head;
        Stack<int> st = [];
        st.Push(slow.val);
        while (fast.next is not null && fast.next.next is not null)
        {
            fast = fast.next.next;
            slow = slow.next;
            st.Push(slow.val);
        }
        slow = slow.next;
        int res = 0;
        while (slow is not null)
        {
            var top = st.Pop();
            res = Math.Max(res, slow.val + top);
            slow = slow.next;
        }
        return res;
    }
}

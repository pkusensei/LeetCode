using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PairSum(ListNode head)
    {
        var slow = head;
        var fast = head;
        Stack<int> st = [];
        while (fast is not null)
        {
            st.Push(slow.val);
            fast = fast.next.next;
            slow = slow.next;
        }
        int res = 0;
        while (slow is not null)
        {
            int top = st.Pop();
            res = int.Max(res, top + slow.val);
            slow = slow.next;
        }
        return res;
    }
}

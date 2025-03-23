using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode RemoveNodes(ListNode head)
    {
        Stack<ListNode> st = [];
        var curr = head;
        while (curr is not null)
        {
            while (st.TryPeek(out var n) && n.val < curr.val) { st.Pop(); }
            st.Push(curr);
            curr = curr.next;
        }
        ListNode dummy = new(0);
        curr = dummy;
        foreach (var item in st.Reverse())
        {
            curr.next = item;
            curr = curr.next;
        }
        curr.next = null;
        return dummy.next;
    }
}

using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode ReverseKGroup(ListNode head, int k)
    {
        var tail = head;
        for (int _ = 0; _ < k; _++)
        {
            if (tail is null) { return head; }
            tail = tail.next;
        }
        var lst = Reverse(head, tail);
        head.next = ReverseKGroup(tail, k);
        return lst;

        static ListNode Reverse(ListNode curr, ListNode tail)
        {
            ListNode prev = null;
            while (curr != tail)
            {
                var next = curr.next;
                curr.next = prev;
                prev = curr;
                curr = next;
            }
            return prev;
        }
    }
}

using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode RotateRight(ListNode head, int k)
    {
        if (head is null || head.next is null || k == 0) { return head; }
        int len = 1;
        ListNode tail = head;
        while (tail.next is not null)
        {
            tail = tail.next;
            len += 1;
        }
        k %= len;
        if (k == 0) { return head; }
        int left_len = len - k;
        ListNode new_tail = head;
        for (int _ = 0; _ < left_len - 1; _++)
        {
            new_tail = new_tail.next;
        }
        ListNode new_head = new_tail.next;
        new_tail.next = null;
        tail.next = head;
        return new_head;
    }
}

using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode ReverseEvenLengthGroups(ListNode head)
    {
        ListNode dummy = new(0, head);
        var prev = dummy;
        var curr = head;
        for (int len = 1; curr is not null; len += 1)
        {
            var tail = curr;
            var count = 1;
            while (count < len && tail is not null && tail.next is not null)
            {
                tail = tail.next;
                count += 1;
            }
            ListNode next_head = tail.next;
            if ((count & 1) == 0)
            {
                tail.next = null;
                prev.next = Reverse(curr);
                prev = curr; // jump forward
                curr.next = next_head;
                curr = next_head;
            }
            else
            {
                prev = tail;
                curr = next_head;
            }
        }
        return dummy.next;

        static ListNode Reverse(ListNode node)
        {
            if (node is null) { return null; }
            ListNode prev = null;
            var curr = node;
            while (curr is not null)
            {
                var temp = curr.next;
                curr.next = prev;
                prev = curr;
                curr = temp;
            }
            return prev;
        }
    }
}


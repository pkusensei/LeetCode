using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode ReverseList(ListNode head)
    {
        ListNode left = null;
        ListNode right = head;
        while (right is not null)
        {
            var temp = right.next;
            right.next = left;
            left = right;
            right = temp;
        }
        return left;
    }

    public ListNode WithRecursion(ListNode head)
    {
        if (head is null || head.next is null) { return head; }
        return Reverse(head).head;

        static (ListNode head, ListNode tail) Reverse(ListNode node)
        {
            if (node.next is null) { return (node, node); }
            var next = node.next;
            node.next = null;
            (var head, var tail) = Reverse(next);
            tail.next = node;
            return (head, node);
        }
    }
}

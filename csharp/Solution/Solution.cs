using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode GetIntersectionNode(ListNode headA, ListNode headB)
    {
        (var tail1, int len1) = FindTail(headA);
        (var tail2, int len2) = FindTail(headB);
        if (!ReferenceEquals(tail1, tail2)) { return null; }
        if (len1 < len2)
        {
            (headA, headB) = (headB, headA);
            (len1, len2) = (len2, len1);
        }
        var curr1 = headA;
        while (len1 > len2)
        {
            curr1 = curr1.next;
            len1 -= 1;
        }
        var curr2 = headB;
        while (!ReferenceEquals(curr1, curr2))
        {
            curr1 = curr1.next;
            curr2 = curr2.next;
        }
        return curr1;

        static (ListNode tail, int count) FindTail(ListNode head)
        {
            var curr = head;
            int count = 0;
            while (curr.next is not null)
            {
                curr = curr.next;
                count += 1;
            }
            return (curr, count);
        }
    }

    public ListNode WithSinglePass(ListNode headA, ListNode headB)
    {
        if (headA is null || headB is null) { return null; }
        var p1 = headA;
        var p2 = headB;
        // Either arrive at intersection or both at null
        while (!ReferenceEquals(p1, p2))
        {
            p1 = p1 is not null ? p1.next : headB;
            p2 = p2 is not null ? p2.next : headA;
        }
        return p1;
    }
}
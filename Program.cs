using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = ListNode.Make([4, 2, 1, 3]);
    var s = SortList(n);
    var a = "[1,2,3,4]";
    Debug.Assert(s.ToString() == a, $"Output:{s}\nExpect:{a}");
}

void Test2()
{
    var n = ListNode.Make([-1, 5, 3, 4, 0]);
    var s = SortList(n);
    var a = "[-1,0,3,4,5]";
    Debug.Assert(s.ToString() == a, $"Output:{s}\nExpect:{a}");
}

ListNode SortList(ListNode head)
{
    if (head is null || head.next is null) { return head; }
    (var slow, var fast) = (head, head);
    ListNode prev = null;
    while (fast is not null && fast.next is not null)
    {
        prev = slow;
        fast = fast.next.next;
        slow = slow.next;
    }
    prev.next = null;
    var left = SortList(head);
    var right = SortList(slow);
    return Merge(left, right);
}

ListNode Merge(ListNode left, ListNode right)
{
    ListNode dummy = new(0);
    var curr = dummy;

    while (left is not null && right is not null)
    {
        if (left.val < right.val)
        {
            curr.next = left;
            left = left.next;
        }
        else
        {
            curr.next = right;
            right = right.next;
        }
        curr = curr.next;
    }
    if (left is not null) { curr.next = left; }
    if (right is not null) { curr.next = right; }
    return dummy.next;
}
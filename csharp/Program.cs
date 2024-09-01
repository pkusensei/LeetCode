using System.Diagnostics;
using LList;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = LList.ListNode.Make([1, 2, 2, 1]);
    var s = IsPalindrome(n);
    Debug.Assert(s, $"Output: {s}\nExpect: {!s}");
}

void Test2()
{
    var n = LList.ListNode.Make([1, 2]);
    var s = IsPalindrome(n);
    Debug.Assert(!s, $"Output: {s}\nExpect: {!s}");
}

bool IsPalindrome(ListNode head)
{
    if (head is null) { return true; }

    (var slow, var fast) = (head, head.next);
    while (fast is not null && fast.next is not null)
    {
        slow = slow.next;
        fast = fast.next.next;
    }
    var tail = Reverse(slow.next);
    slow.next = null;
    while (tail is not null)
    {
        if (head.val != tail.val) { return false; }
        head = head.next;
        tail = tail.next;
    }
    return true;
}

ListNode Reverse(ListNode curr)
{
    ListNode prev = null;
    while (curr is not null)
    {
        var temp = curr.next;
        curr.next = prev;
        prev = curr;
        curr = temp;
    }
    return prev;
}

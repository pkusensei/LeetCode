using System.Diagnostics;

Test1();
Test2();
Test3();

Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = ListNode.Make([3, 2, 0, -4], 1);
    var s = DetectCycle(n);
    Debug.Assert(s.val == 2);
}

void Test2()
{
    var n = ListNode.Make([1, 2], 0);
    var s = DetectCycle(n);
    Debug.Assert(s.val == 1);
}

void Test3()
{
    var n = ListNode.Make([1], -1);
    var s = DetectCycle(n);
    Debug.Assert(s is null);
}

ListNode DetectCycle(ListNode head)
{
    if (head is null) { return null; }

    (var slow, var fast) = (head, head);
    var hasCycle = false;
    while (fast is not null && fast.next is not null)
    {
        slow = slow.next;
        fast = fast.next.next;
        if (slow == fast) { hasCycle = true; break; }
    }
    if (!hasCycle) { return null; }

    slow = head;
    while (slow != fast)
    {
        slow = slow.next;
        fast = fast.next;
    }
    return slow;
}

bool HasCycle(ListNode head)
{
    if (head is null) { return false; }
    (var slow, var fast) = (head, head);
    while (fast is not null && fast.next is not null)
    {
        slow = slow.next;
        fast = fast.next.next;
        if (slow == fast) { return true; }
    }
    return false;
}
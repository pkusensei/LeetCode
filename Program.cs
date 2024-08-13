using System.Diagnostics;

Test1();
Test2();
Test3();

Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = ListNode.Make([3, 2, 0, -4], 1);
    Debug.Assert(HasCycle(n));
}

void Test2()
{
    var n = ListNode.Make([1, 2], 0);
    Debug.Assert(HasCycle(n));
}

void Test3()
{
    var n = ListNode.Make([1], -1);
    Debug.Assert(!HasCycle(n));
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
using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
}

void Test2()
{
}

ListNode GetIntersectionNode(ListNode headA, ListNode headB)
{
    if (headA is null || headB is null) { return null; }

    (var a, var b) = (headA, headB);
    while (a != b)
    {
        a = a is null ? headB : a.next;
        b = b is null ? headA : b.next;
    }
    return a;
}
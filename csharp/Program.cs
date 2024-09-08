using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var a = ListNode.Make([1, 2, 3, 4, 5]);
    var b = OddEvenList(a);
    var c = "[1,3,5,2,4]";
    Debug.Assert(b.ToString() == c, $"Output: {b}\nExpect: {c}");
}

void Test2()
{
    var a = ListNode.Make([2, 1, 3, 5, 6, 4, 7]);
    var b = OddEvenList(a);
    var c = "[2,3,6,7,1,5,4]";
    Debug.Assert(b.ToString() == c, $"Output: {b}\nExpect: {c}");
}

ListNode OddEvenList(ListNode head)
{
    if (head is null) { return null; }
    var h2 = head.next;
    (var curr1, var curr2) = (head, h2);
    while (curr2 is not null && curr2.next is not null)
    {
        curr1.next = curr2.next;
        curr1 = curr1.next;

        curr2.next = curr1.next;
        curr2 = curr2.next;
    }
    curr1.next = h2;
    return head;
}

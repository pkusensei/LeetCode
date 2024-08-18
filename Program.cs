using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var node = ListNode.Make([1, 2, 6, 3, 4, 5, 6]);
    var s = RemoveElements(node, 6);
    var a = "[1,2,3,4,5]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var node = ListNode.Make([7, 7, 7, 7]);
    var s = RemoveElements(node, 7);
    var a = "[]";
    Debug.Assert(s is null);
}

ListNode RemoveElements(ListNode head, int val)
{
    if (head is null) { return null; }

    ListNode dummy = new(0, head);
    var curr = dummy;
    while (curr.next is not null)
    {
        if (curr.next.val == val)
        {
            curr.next = curr.next.next;
        }
        else { curr = curr.next; }
    }
    return dummy.next;
}
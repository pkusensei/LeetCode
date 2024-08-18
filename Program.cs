using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var node = ListNode.Make([5, 4, 3, 2, 1]);
    var s = ReverseList(node);
    var a = "[1,2,3,4,5]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var node = ListNode.Make([1, 2]);
    var s = ReverseList(node);
    var a = "[2,1]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

ListNode ReverseList(ListNode head)
{
    if (head is null) { return null; }

    ListNode dummy = new(0);
    var curr = head;
    while (curr.next is not null)
    {
        var temp = curr;
        curr = curr.next;
        temp.next = dummy.next;
        dummy.next = temp;
    }
    curr.next = dummy.next;
    dummy.next = curr;
    return dummy.next;
}
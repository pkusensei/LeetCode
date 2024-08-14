using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = ListNode.Make([4, 2, 1, 3]);
    var s = InsertionSortList(n);
    var a = "[1,2,3,4]";
    Debug.Assert(s.ToString() == a, $"Output:{s}\nExpect:{a}");
}

void Test2()
{
    var n = ListNode.Make([-1, 5, 3, 4, 0]);
    var s = InsertionSortList(n);
    var a = "[-1,0,3,4,5]";
    Debug.Assert(s.ToString() == a, $"Output:{s}\nExpect:{a}");
}

ListNode InsertionSortList(ListNode head)
{
    if (head is null) { return null; }
    (ListNode dummy1, ListNode dummy2) = (new(0, head), new(0, head.next));
    head.next = null;
    while (dummy2.next is not null)
    {
        var curr = dummy2.next;
        dummy2.next = curr.next;

        if (curr.val < dummy1.next.val)
        {
            curr.next = dummy1.next;
            dummy1.next = curr;
        }
        else
        {
            (var prev, var next) = (dummy1, dummy1.next);
            while (next is not null && next.val < curr.val)
            {
                prev = next;
                next = next.next;
            }
            curr.next = next;
            prev.next = curr;
        }
    }
    return dummy1.next;
}
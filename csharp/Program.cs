using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Test3();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = ListNode.Make([1, 2, 3, 4, 5]);
    var s = ModifiedList([1, 2, 3], n);
    var a = "[4,5]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var n = ListNode.Make([1, 2, 1, 2, 1, 2]);
    var s = ModifiedList([1], n);
    var a = "[2,2,2]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

void Test3()
{
    var n = ListNode.Make([1, 2, 3, 4]);
    var s = ModifiedList([5], n);
    var a = "[1,2,3,4]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

ListNode ModifiedList(int[] nums, ListNode head)
{
    if (head is null) { return null; }
    if (nums.Length == 0) { return head; }
    ListNode dummy = new(0, head);
    var s = nums.ToHashSet();
    var curr = dummy;
    while (curr.next is not null)
    {
        if (s.Contains(curr.next.val))
        {
            curr.next = curr.next.next;
        }
        else
        {
            curr = curr.next;
        }
    }
    return dummy.next;
}
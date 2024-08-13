using System.Diagnostics;

Test1();
Test2();
Test3();

Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = ListNode.Make([1, 2, 3, 4]);
    ReorderList(n);
    var a = "[1,4,2,3]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
    var n = ListNode.Make([1, 2, 3, 4, 5]);
    ReorderList(n);
    var a = "[1,5,2,4,3]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test3()
{
}

void ReorderList(ListNode head)
{
    if (head is null) { return; }

    var mid = FindMiddleNode(head);
    var right = mid.next;
    mid.next = null;
    var stack = new Stack<ListNode>();
    while (right is not null)
    {
        stack.Push(right);
        right = right.next;
    }
    var left = head;
    while (stack.TryPop(out var node))
    {
        var temp = left.next;
        left.next = node;
        node.next = temp;
        left = temp;
    }
}

ListNode FindMiddleNode(ListNode head)
{
    var curr = head;
    if (curr is null || curr.next is null) { return curr; }

    (var slow, var fast) = (curr, curr);
    while (fast.next is not null && fast.next.next is not null)
    {
        slow = slow.next;
        fast = fast.next.next;
    }
    return slow;
}

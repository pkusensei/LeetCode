// See https://aka.ms/new-console-template for more information

using System.Diagnostics;

Test1();
Test2();
Test3();
Test4();


void Test1()
{
    ListNode n1 = new(1, new(2, new(3, new(4, new(5)))));
    ListNode n2 = new(1, new(2, new(3, new(5))));
    var ans = RemoveNthFromEnd(n1, 2);
    Console.WriteLine(ans);
    Console.WriteLine(n2);
}

void Test2()
{
    ListNode n = new(1);
    Debug.Assert(RemoveNthFromEnd(n, 1) == null);
}

void Test3()
{
    ListNode n1 = new(1, new(2));
    ListNode n2 = new(1);
    var ans = RemoveNthFromEnd(n1, 1);
    Console.WriteLine(ans);
    Console.WriteLine(n2);

}

void Test4()
{
    ListNode n1 = new(1, new(2));
    ListNode n2 = new(2);
    var ans = RemoveNthFromEnd(n1, 2);
    Console.WriteLine(ans);
    Console.WriteLine(n2);
}

ListNode RemoveNthFromEnd(ListNode head, int n)
{
    ListNode lst = new(0, head);
    (ListNode node, int step) fast = (lst, 0);
    (ListNode node, int step) slow = (lst, 0);
    while (fast.node.next is not null)
    {
        fast.node = fast.node.next;
        fast.step += 1;
        if (slow.step + n < fast.step)
        {
            slow.step += 1;
            slow.node = slow.node.next;
        }
    }
    slow.node.next = slow.node.next.next;
    return lst.next;
}


class ListNode
{
    public int val;
    public ListNode next;
    public ListNode(int val = 0, ListNode next = null)
    {
        this.val = val;
        this.next = next;
    }
}

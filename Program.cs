// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");

Test2();

void Test1()
{
    ListNode l1 = new(2, new(4, new(3)));
    ListNode l2 = new(5, new(6, new(4)));
    ListNode l3 = new(7, new(0, new(8)));
    ListNode ans = AddTwoNumbers(l1, l2);
    System.Diagnostics.Debug.Assert(l3 == ans);
}

void Test2()
{
    ListNode l1 = new(5);
    ListNode l2 = new(5);
    ListNode l3 = new(0, new(1));
    ListNode ans = AddTwoNumbers(l1, l2);
    System.Diagnostics.Debug.Assert(l3 == ans);

}

ListNode AddTwoNumbers(ListNode l1, ListNode l2)
{
    var carry = 0;
    ListNode head = null;
    ListNode curr = null;
    while (l1 is not null && l2 is not null)
    {
        var v1 = l1.val;
        l1 = l1.next;
        var v2 = l2.val;
        l2 = l2.next;

        var sum = v1 + v2 + carry;
        carry = sum / 10;
        var node = new ListNode(sum % 10);
        if (head is null)
        {
            head = node;
            curr = head;
        }
        else
        {
            curr.next = node;
            curr = curr.next;
        }
    }
    var prev = curr;
    if (l1 is not null)
    {
        curr.next = l1;
    }
    else if (l2 is not null)
    {
        curr.next = l2;
    }
    curr = curr.next;
    while (carry > 0 && curr is not null)
    {
        var val = curr.val;
        curr.val = (val + carry) % 10;
        carry = (val + carry) / 10;
        prev = curr;
        curr = curr.next;
    }
    if (carry > 0 && curr is null)
    {
        prev.next = new ListNode(carry);
    }

    return head;
}

public class ListNode
{
    public int val;
    public ListNode next;
    public ListNode(int val = 0, ListNode next = null)
    {
        this.val = val;
        this.next = next;
    }
}

